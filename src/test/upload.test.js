/**
 * upload.js 模块测试
 * 测试上传相关的核心功能
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
  getActiveUploads: vi.fn(),
  setActiveUploads: vi.fn()
}))

vi.mock("./download.js", () => ({
  formatFileSize: vi.fn((bytes) => {
    if (bytes === 0) return "0 B"
    const units = ["B", "KB", "MB", "GB", "TB"]
    const i = Math.floor(Math.log(bytes) / Math.log(1024))
    return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${units[i]}`
  })
}))

import { 
  uploadFile, 
  getUploadProgress, 
  extractFileName 
} from "../components/data/upload.js"
import { invoke } from "@tauri-apps/api/core"
import { showToast } from "../layout/showToast.js"
import { getActiveUploads, setActiveUploads } from "./storage.js"

describe("extractFileName 函数测试", () => {
  it("应该从 Windows 路径提取文件名", () => {
    expect(extractFileName("C:\\Users\\test\\file.txt")).toBe("file.txt")
    expect(extractFileName("D:\\data\\folder\\doc.pdf")).toBe("doc.pdf")
  })

  it("应该从 Unix 路径提取文件名", () => {
    expect(extractFileName("/home/user/file.txt")).toBe("file.txt")
    expect(extractFileName("/var/www/index.html")).toBe("index.html")
  })

  it("应该处理没有路径分隔符的文件名", () => {
    expect(extractFileName("file.txt")).toBe("file.txt")
    expect(extractFileName("README.md")).toBe("README.md")
  })

  it("应该处理混合路径分隔符", () => {
    expect(extractFileName("C:/Users\\test/file.txt")).toBe("file.txt")
  })

  it("应该处理空路径", () => {
    expect(extractFileName("")).toBe("")
  })
})

describe("uploadFile 函数测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该成功上传文件", async () => {
    const filePath = "C:\\test\\file.zip"
    invoke.mockResolvedValue("上传成功，upload_id: 123")

    const result = await uploadFile(filePath)

    expect(invoke).toHaveBeenCalledWith("upload_file", { filePath: "C:\\test\\file.zip" })
    expect(result).toBe("上传成功，upload_id: 123")
    expect(showToast).toHaveBeenCalledWith("文件上传已开始", "#10b981")
  })

  it("应该显示开始上传的提示", async () => {
    invoke.mockResolvedValue("完成")

    await uploadFile("test.zip")

    expect(showToast).toHaveBeenCalledWith("开始上传文件...", "#3b82f6")
  })

  it("应该处理网络错误", async () => {
    invoke.mockRejectedValue("网络错误")

    await expect(uploadFile("test.zip")).rejects.toThrow("上传失败")
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("网络连接失败"),
      "#ef4444"
    )
  })

  it("应该处理蓝牙设备连接失败", async () => {
    invoke.mockRejectedValue("获取设备 ID 失败")

    await expect(uploadFile("test.zip")).rejects.toThrow()
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("蓝牙设备连接失败"),
      "#ef4444"
    )
  })

  it("应该处理 TOTP 验证失败", async () => {
    invoke.mockRejectedValue("获取 TOT P 失败")

    await expect(uploadFile("test.zip")).rejects.toThrow()
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("TOTP 验证失败"),
      "#ef4444"
    )
  })

  it("应该处理创建上传任务失败", async () => {
    invoke.mockRejectedValue("创建上传任务失败")

    await expect(uploadFile("test.zip")).rejects.toThrow()
    expect(showToast).toHaveBeenCalledWith(
      expect.stringContaining("创建上传任务失败"),
      "#ef4444"
    )
  })
})

describe("getUploadProgress 函数测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该正确获取上传进度", async () => {
    const mockProgress = {
      upload_id: "123",
      filename: "test.zip",
      total_size: 1048576,
      uploaded: 524288,
      status: "Uploading",
      chunks_total: 4,
      chunks_completed: 2,
      speed_kbps: 1024,
      progress_percentage: 50
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getUploadProgress("123")

    expect(result.progress_percentage).toBe(50)
    expect(result.formatted_total_size).toBe("1.00 MB")
    expect(result.formatted_uploaded).toBe("512.00 KB")
    expect(result.chunks_info).toBe("分片 2/4")
  })

  it("应该自动计算进度百分比（当后端未返回时）", async () => {
    const mockProgress = {
      upload_id: "123",
      total_size: 2000,
      uploaded: 500,
      status: "Uploading"
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getUploadProgress("123")

    expect(result.progress_percentage).toBe(25)
  })

  it("应该处理获取进度失败的情况", async () => {
    invoke.mockRejectedValue("获取失败")

    const result = await getUploadProgress("123")

    expect(result.status).toBe("Error")
    expect(result.progress_percentage).toBe(0)
    expect(result.formatted_total_size).toBe("未知大小")
    expect(result.upload_id).toBe("123")
  })

  it("应该处理 total_size 为 0 的情况", async () => {
    const mockProgress = {
      upload_id: "123",
      total_size: 0,
      uploaded: 0,
      status: "Pending"
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getUploadProgress("123")

    expect(result.progress_percentage).toBe(0)
    expect(result.formatted_total_size).toBe("未知大小")
  })

  it("应该包含估算剩余时间字段", async () => {
    const mockProgress = {
      upload_id: "123",
      total_size: 1000,
      uploaded: 500,
      status: "Uploading"
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getUploadProgress("123")

    expect(result.estimated_remaining).toBe("计算中...")
  })

  it("应该处理分片上传进度", async () => {
    const mockProgress = {
      upload_id: "123",
      total_size: 4096,
      uploaded: 4096,
      status: "Completed",
      chunks_total: 4,
      chunks_completed: 4
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getUploadProgress("123")

    expect(result.progress_percentage).toBe(100)
    expect(result.status).toBe("Completed")
    expect(result.chunks_info).toBe("分片 4/4")
  })
})

describe("边界情况测试", () => {
  it("应该处理非常大的文件路径", () => {
    const longPath = "C:\\Users\\test\\" + "a".repeat(200) + "\\file.txt"
    expect(extractFileName(longPath)).toBe("file.txt")
  })

  it("应该处理特殊字符的文件名", () => {
    expect(extractFileName("C:\\test\\文件 123.txt")).toBe("文件 123.txt")
    expect(extractFileName("D:\\data\\test-file_v2.0.zip")).toBe("test-file_v2.0.zip")
  })

  it("应该处理上传进度为 100% 的情况", async () => {
    const mockProgress = {
      upload_id: "123",
      total_size: 1024,
      uploaded: 1024,
      status: "Completed",
      progress_percentage: 100
    }
    invoke.mockResolvedValue(mockProgress)

    const result = await getUploadProgress("123")

    expect(result.progress_percentage).toBe(100)
    expect(result.status).toBe("Completed")
  })
})
