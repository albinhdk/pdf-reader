import { invoke } from '@tauri-apps/api/core';

/**
 * PDF分片加载器
 * 实现大文件的分片加载和内存优化
 */
export class ChunkedPdfLoader {
  private filePath: string;
  private fileSize: number = 0;
  private chunkSize: number;
  private chunks: Map<number, Uint8Array> = new Map();
  private maxCachedChunks: number;
  private loadingPromises: Map<number, Promise<Uint8Array>> = new Map();

  constructor(filePath: string, options: {
    chunkSize?: number;
    maxCachedChunks?: number;
  } = {}) {
    this.filePath = filePath;
    this.chunkSize = options.chunkSize || 1024 * 1024; // 默认1MB
    this.maxCachedChunks = options.maxCachedChunks || 20;
  }

  /**
   * 初始化加载器，获取文件信息
   */
  async initialize(): Promise<void> {
    try {
      const [fileSize, sizeStr] = await invoke('get_pdf_file_info', { path: this.filePath }) as [number, string];
      this.fileSize = fileSize;
      console.log(`PDF文件信息: ${sizeStr}, 将使用分片加载 (分片大小: ${this.chunkSize / 1024}KB)`);
      
      // 根据文件大小调整策略
      this.adjustStrategyByFileSize();
    } catch (error) {
      throw new Error(`获取文件信息失败: ${error}`);
    }
  }

  /**
   * 根据文件大小调整加载策略
   */
  private adjustStrategyByFileSize(): void {
    const fileSizeMB = this.fileSize / (1024 * 1024);
    
    if (fileSizeMB > 100) {
      this.chunkSize = 2 * 1024 * 1024; // 2MB chunks
      this.maxCachedChunks = 10;
      console.log('大文件策略: 2MB分片, 最多缓存10个分片');
    } else if (fileSizeMB > 50) {
      this.chunkSize = 1 * 1024 * 1024; // 1MB chunks
      this.maxCachedChunks = 20;
      console.log('中等文件策略: 1MB分片, 最多缓存20个分片');
    } else {
      this.chunkSize = 512 * 1024; // 512KB chunks
      this.maxCachedChunks = 50;
      console.log('小文件策略: 512KB分片, 最多缓存50个分片');
    }
  }

  /**
   * 加载指定的数据块
   */
  async loadChunk(chunkIndex: number): Promise<Uint8Array> {
    // 检查缓存
    if (this.chunks.has(chunkIndex)) {
      return this.chunks.get(chunkIndex)!;
    }

    // 检查是否正在加载
    if (this.loadingPromises.has(chunkIndex)) {
      return await this.loadingPromises.get(chunkIndex)!;
    }

    // 开始加载
    const loadPromise = this.doLoadChunk(chunkIndex);
    this.loadingPromises.set(chunkIndex, loadPromise);

    try {
      const chunk = await loadPromise;
      this.chunks.set(chunkIndex, chunk);
      this.loadingPromises.delete(chunkIndex);
      
      // 清理过多的缓存
      this.cleanupCache();
      
      // 暂时禁用预加载，避免与PDF.js的请求策略冲突
      // this.preloadAdjacentChunks(chunkIndex);
      
      return chunk;
    } catch (error) {
      this.loadingPromises.delete(chunkIndex);
      throw error;
    }
  }

  /**
   * 实际执行分片加载
   */
  private async doLoadChunk(chunkIndex: number): Promise<Uint8Array> {
    const offset = chunkIndex * this.chunkSize;
    
    if (offset >= this.fileSize) {
      return new Uint8Array(0);
    }

    try {
      const data = await invoke('read_pdf_file_chunked', {
        path: this.filePath,
        chunkSize: this.chunkSize,
        offset: offset
      }) as number[];
      
      return new Uint8Array(data);
    } catch (error) {
      console.error(`加载分片 ${chunkIndex} 失败:`, error);
      throw error;
    }
  }



  /**
   * 清理过多的缓存
   */
  private cleanupCache(): void {
    if (this.chunks.size <= this.maxCachedChunks) {
      return;
    }

    // 简单的LRU策略：删除最早的分片
    const chunksToDelete = this.chunks.size - this.maxCachedChunks;
    const chunkIndices = Array.from(this.chunks.keys()).sort((a, b) => a - b);
    
    for (let i = 0; i < chunksToDelete; i++) {
      const chunkIndex = chunkIndices[i];
      this.chunks.delete(chunkIndex);
      console.log(`清理缓存分片: ${chunkIndex}`);
    }
  }

  /**
   * 获取指定范围的数据
   */
  async getRange(start: number, length: number): Promise<Uint8Array> {
    const result = new Uint8Array(length);
    let resultOffset = 0;
    
    while (resultOffset < length) {
      const currentPos = start + resultOffset;
      const chunkIndex = Math.floor(currentPos / this.chunkSize);
      const chunkOffset = currentPos % this.chunkSize;
      
      const chunk = await this.loadChunk(chunkIndex);
      
      const copyLength = Math.min(
        chunk.length - chunkOffset,
        length - resultOffset
      );
      
      if (copyLength > 0) {
        result.set(
          chunk.subarray(chunkOffset, chunkOffset + copyLength),
          resultOffset
        );
        resultOffset += copyLength;
      } else {
        break; // 到达文件末尾
      }
    }
    
    return result.subarray(0, resultOffset);
  }

  /**
   * 创建虚拟ArrayBuffer，支持PDF.js的范围请求
   */
  createVirtualArrayBuffer(): ChunkedArrayBuffer {
    return new ChunkedArrayBuffer(this);
  }

  /**
   * 获取文件大小
   */
  getFileSize(): number {
    return this.fileSize;
  }

  /**
   * 清理所有缓存
   */
  cleanup(): void {
    this.chunks.clear();
    this.loadingPromises.clear();
    console.log('PDF分片加载器已清理');
  }
}

/**
 * 虚拟ArrayBuffer，支持分片加载
 */
export class ChunkedArrayBuffer {
  private loader: ChunkedPdfLoader;
  public readonly byteLength: number;

  constructor(loader: ChunkedPdfLoader) {
    this.loader = loader;
    this.byteLength = loader.getFileSize();
  }

  /**
   * 获取指定范围的数据
   */
  async slice(start: number, end?: number): Promise<ArrayBuffer> {
    const actualEnd = end !== undefined ? Math.min(end, this.byteLength) : this.byteLength;
    const length = actualEnd - start;
    
    if (length <= 0) {
      return new ArrayBuffer(0);
    }

    const data = await this.loader.getRange(start, length);
    return data.buffer.slice(data.byteOffset, data.byteOffset + data.byteLength);
  }

  /**
   * 同步slice（用于兼容性）
   */
  sliceSync(_start: number, _end?: number): ArrayBuffer {
    // 这是一个同步方法，但实际上我们需要异步加载
    // 这里返回一个空的ArrayBuffer，实际使用中应该避免调用此方法
    console.warn('ChunkedArrayBuffer.sliceSync 被调用，这可能导致问题');
    return new ArrayBuffer(0);
  }
}