/**
 * download.js 模块测试
 * 测试下载相关的核心功能
 */
import { describe, it, expect, vi, beforeEach } from "vitest"

// 模拟依赖模块
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn()
}))

vi.mock("../layout/showToast.js", () => ({
  showToast: vi.fn()
}))

vi.mock("./storage.js", () => ({
  getActiveDownloads: vi.fn(),
  setActiveDownloads: vi.fn()
}))

import { formatFileSize, downloadFile, getDownloadProgress } from "../components/data/download.js"
import { invoke } from "@tauri-apps/api/core"
import { showToast } from "../layout/showToast.js"
import { getActiveDownloads, setActiveDownloads } from "./storage.js"

describe("formatFileSize 函数测试", () => {
  it("应该正确处理 0 字节", () => {
    expect(formatFileSize(0)).toBe("0 B")
  })

  it("应该正确格式化小文件（字节级别）", () => {
    expect(formatFileSize(100)).toBe("100.00 B")
    expect(formatFileSize(512)).toBe("512.00 B")
  })

  it("应该正确格式化 KB 级别的文件", () => {
    expect(formatFileSize(1024)).toBe("1.00 KB")
    expect(formatFileSize(1536)).toBe("1.50 KB")
    expect(formatFileSize(5120)).toBe("5.00 KB")
  })

  it("应该正确格式化 MB 级别的文件", () => {
    expect(formatFileSize(1048576)).toBe("1.00 MB")
    expect(formatFileSize(1572864)).toBe("1.50 MB")
    expect(formatFileSize(5242880)).toBe("5.00 MB")
  })

  it("应该正确格式化 GB 级别的文件", () => {
    expect(formatFileSize(1073741824)).toBe("1.00 GB")
    expect(formatFileSize(1610612736)).toBe("1.50 GB")
  })

  it("应该正确格式化 TB 级别的文件", () => {
    expect(formatFileSize(1099511627776)).toBe("1.00 TB")
  })
})

describe("downloadFile 函数测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    getActiveDownloads.mockResolvedValue([])
    setActiveDownloads.mockResolvedValue()
  })

  it("应该成功下载文件", async () => {
    const fileId = "abc123"
    invoke.mockResolvedValue("下载成功")

    const result = await downloadFile(fileId)

    expect(invoke).toHaveBeenCalledWith("download_file", { fileId: "abc123" })
    expect(result).toBe("下载成功")
    expect(showToast).not.toHaveBeenCalled()
  })

  it("应该保存下载记录到本地存储", async () => {
    const fileId = "test123"
    invoke.mockResolvedValue("完成")
    getActiveDownloads.mockResolvedValue(["old1"])

    await downloadFile(fileId)

    expect(setActiveDownloads).toHaveBeenCalledWith(["old1", "test123"])
  })

  it("应该处理网络错误", async () => {
    const fileId = "fail1"
    invoke.mockRejectedValue("网络错误")

    await expect(downloadFile(fileId)).rejects.toThrow("下载失败")
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("网络连接失败"),
      "#ef4444"
    )
  })

  it("应该处理蓝牙设备连接失败", async () => {
    invoke.mockRejectedValue("获取设备 ID 失败")

    await expect(downloadFile("test")).rejects.toThrow()
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("蓝牙设备连接失败"),
      "#ef4444"
    )
  })

  it("应该处理 TOTP 验证失败", async () => {
    invoke.mockRejectedValue("获取 TOT P 失败")

    await expect(downloadFile("test")).rejects.toThrow()
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("TOTP 验证失败"),
      "#ef4444"
    )
  })

  it("应该处理超时错误", async () => {
    invoke.mockRejectedValue("下载超时")

    await expect(downloadFile("test")).rejects.toThrow()
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("下载超时"),
      "#ef4444"
    )
  })
})

describe("getDownloadProgress 函数测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该正确获取下载进度", async () => {
    const mockProgress = {
      file_id: "abc123",
      file_name: "test.zip",
      total_size: 1048576,
      downloaded: 524288,
      status: "Downloading",
      chunks_total: 4,
      chunks_completed: 2,
      speed_kbps: 1024,
      progress_percentage: 50
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getDownloadProgress("abc123")

    expect(result.progress_percentage).toBe(50)
    expect(result.formatted_total_size).toBe("1.00 MB")
    expect(result.formatted_downloaded).toBe("512.00 KB")
    expect(result.chunks_info).toBe("分片 2/4")
  })

  it("应该自动计算进度百分比（当后端未返回时）", async () => {
    const mockProgress = {
      file_id: "abc123",
      total_size: 1000,
      downloaded: 250,
      status: "Downloading"
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getDownloadProgress("abc123")

    expect(result.progress_percentage).toBe(25)
  })

  it("应该处理获取进度失败的情况", async () => {
    invoke.mockRejectedValue("获取失败")

    const result = await getDownloadProgress("abc123")

    expect(result.status).toBe("Error")
    expect(result.progress_percentage).toBe(0)
    expect(result.formatted_total_size).toBe("未知大小")
  })

  it("应该处理 total_size 为 0 的情况", async () => {
    const mockProgress = {
      file_id: "abc123",
      total_size: 0,
      downloaded: 0,
      status: "Pending"
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getDownloadProgress("abc123")

    expect(result.progress_percentage).toBe(0)
    expect(result.formatted_total_size).toBe("未知大小")
  })

  it("应该处理分片信息", async () => {
    const mockProgress = {
      file_id: "abc123",
      total_size: 2048,
      downloaded: 2048,
      status: "Completed",
      chunks_total: 2,
      chunks_completed: 2
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getDownloadProgress("abc123")

    expect(result.progress_percentage).toBe(100)
    expect(result.chunks_info).toBe("分片 2/2")
  })
})
