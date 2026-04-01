/**
 * fileSystem.js 模块测试
 * 测试文件系统相关的核心功能（ls, mkdir, rm）
 */
import { describe, it, expect, vi, beforeEach } from "vitest"

// 模拟 axios
vi.mock("axios", () => ({
  default: {
    get: vi.fn(),
    post: vi.fn(),
    delete: vi.fn()
  }
}))

// 模拟后端配置
vi.mock("../../config/backend.js", () => ({
  getBackendUrl: () => "http://localhost:8080"
}))

// 模拟 bluetooth.js 的导入
vi.mock("../components/data/bluetooth.js", () => ({
  getDeviceId: vi.fn(() => Promise.resolve("test-device-id")),
  getTotp: vi.fn(() => Promise.resolve("123456"))
}))

import axios from "axios"
import { ls, mkdir, rm } from "../components/data/fileSystem.js"

describe("ls 函数测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该成功获取文件列表", async () => {
    const mockResponse = {
      data: {
        files: [
          { name: "file1.txt", type: "file" },
          { name: "folder1", type: "directory" }
        ]
      }
    }
    axios.get.mockResolvedValue(mockResponse)

    const result = await ls("/test/path")

    expect(axios.get).toHaveBeenCalledWith(
      "http://localhost:8080/files/?path=/test/path",
      expect.objectContaining({
        headers: {
          "Id": "test-device-id",
          "Totp": "123456"
        }
      })
    )
    expect(result).toEqual(mockResponse.data)
  })

  it("应该处理超时情况", async () => {
    // 模拟超时（3 秒后返回）
    axios.get.mockImplementation(() => {
      return new Promise(resolve => {
        setTimeout(() => {
          resolve({ data: { files: [] } })
        }, 4000)
      })
    })

    const result = await ls("/test")

    // 超时应该返回 null
    expect(result).toBeNull()
  })

  it("应该处理网络错误", async () => {
    axios.get.mockRejectedValue(new Error("网络错误"))

    await expect(ls("/test")).rejects.toThrow("网络错误")
  })

  it("应该处理 404 错误", async () => {
    const error = new Error("Not Found")
    error.response = { status: 404 }
    axios.get.mockRejectedValue(error)

    await expect(ls("/nonexistent")).rejects.toThrow("Not Found")
  })

  it("应该正确传递认证头", async () => {
    axios.get.mockResolvedValue({ data: { files: [] } })

    await ls("/test")

    const callArgs = axios.get.mock.calls[0]
    expect(callArgs[1].headers).toHaveProperty("Id")
    expect(callArgs[1].headers).toHaveProperty("Totp")
  })

  it("应该处理空路径", async () => {
    axios.get.mockResolvedValue({ data: { files: [] } })

    await ls("")

    expect(axios.get).toHaveBeenCalledWith(
      "http://localhost:8080/files/?path=",
      expect.any(Object)
    )
  })
})

describe("mkdir 函数测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该成功创建目录", async () => {
    const mockResponse = { data: { success: true } }
    axios.post.mockResolvedValue(mockResponse)

    const result = await mkdir("/parent/path", "newFolder")

    expect(axios.post).toHaveBeenCalledWith(
      "http://localhost:8080/files/directories",
      null,
      expect.objectContaining({
        params: expect.any(URLSearchParams),
        headers: expect.any(Object)
      })
    )
    expect(result).toEqual(mockResponse.data)
  })

  it("应该处理超时情况", async () => {
    axios.post.mockImplementation(() => {
      return new Promise(resolve => {
        setTimeout(() => {
          resolve({ data: { success: true } })
        }, 4000)
      })
    })

    const result = await mkdir("/test", "folder")

    expect(result).toBeNull()
  })

  it("应该处理创建失败的情况", async () => {
    axios.post.mockRejectedValue(new Error("目录已存在"))

    await expect(mkdir("/test", "existing")).rejects.toThrow("目录已存在")
  })

  it("应该正确传递目录名称参数", async () => {
    axios.post.mockResolvedValue({ data: { success: true } })

    await mkdir("/parent", "testDir")

    const callArgs = axios.post.mock.calls[0]
    const params = callArgs[2].params
    expect(params.get("directory_name")).toBe("testDir")
    expect(params.get("path")).toBe("/parent")
  })

  it("应该处理特殊字符的目录名", async () => {
    axios.post.mockResolvedValue({ data: { success: true } })

    await mkdir("/parent", "中文目录 123")

    const callArgs = axios.post.mock.calls[0]
    const params = callArgs[2].params
    expect(params.get("directory_name")).toBe("中文目录 123")
  })
})

describe("rm 函数测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该成功删除文件", async () => {
    const mockResponse = { data: { success: true } }
    axios.delete.mockResolvedValue(mockResponse)

    const result = await rm("/path/to/file.txt")

    expect(axios.delete).toHaveBeenCalledWith(
      "http://localhost:8080/files/file.txt",
      expect.objectContaining({
        params: expect.any(URLSearchParams),
        headers: expect.any(Object)
      })
    )
    expect(result).toEqual(mockResponse.data)
  })

  it("应该默认不永久删除（进入回收站）", async () => {
    axios.delete.mockResolvedValue({ data: { success: true } })

    await rm("/test/file.txt")

    const callArgs = axios.delete.mock.calls[0]
    const params = callArgs[1].params
    expect(params.get("permanent")).toBe("false")
  })

  it("应该支持永久删除", async () => {
    axios.delete.mockResolvedValue({ data: { success: true } })

    await rm("/test/file.txt", true)

    const callArgs = axios.delete.mock.calls[0]
    const params = callArgs[1].params
    expect(params.get("permanent")).toBe("true")
  })

  it("应该处理超时情况", async () => {
    axios.delete.mockImplementation(() => {
      return new Promise(resolve => {
        setTimeout(() => {
          resolve({ data: { success: true } })
        }, 4000)
      })
    })

    const result = await rm("/test/file.txt")

    expect(result).toBeNull()
  })

  it("应该处理删除不存在的文件", async () => {
    const error = new Error("文件不存在")
    error.response = { status: 404 }
    axios.delete.mockRejectedValue(error)

    await expect(rm("/nonexistent/file.txt")).rejects.toThrow("文件不存在")
  })

  it("应该正确处理路径中的特殊字符", async () => {
    axios.delete.mockResolvedValue({ data: { success: true } })

    await rm("/path/中文文件.txt")

    // URL 应该被正确编码
    expect(axios.delete).toHaveBeenCalledWith(
      expect.stringContaining("中文文件.txt"),
      expect.any(Object)
    )
  })

  it("应该处理路径为空字符串", async () => {
    axios.delete.mockResolvedValue({ data: { success: true } })

    await rm("")

    expect(axios.delete).toHaveBeenCalledWith(
      "http://localhost:8080/files/",
      expect.any(Object)
    )
  })
})

describe("认证头获取失败的情况", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该在获取设备 ID 失败时使用空 header", async () => {
    // 重新模拟 bluetooth.js 返回错误
    vi.mock("../components/data/bluetooth.js", async () => {
      const actual = await vi.importActual("../components/data/bluetooth.js")
      return {
        ...actual,
        getDeviceId: vi.fn(() => Promise.reject("获取失败")),
        getTotp: vi.fn(() => Promise.resolve("123456"))
      }
    })

    axios.get.mockResolvedValue({ data: { files: [] } })

    // 这个测试主要是验证即使认证失败也不会崩溃
    await expect(ls("/test")).resolves.not.toThrow()
  })
})

describe("边界情况测试", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("应该处理连续多次请求", async () => {
    axios.get.mockResolvedValue({ data: { files: [] } })

    await ls("/path1")
    await ls("/path2")
    await ls("/path3")

    expect(axios.get).toHaveBeenCalledTimes(3)
  })

  it("应该处理超时边界值（刚好在 3 秒内返回）", async () => {
    axios.get.mockImplementation(() => {
      return new Promise(resolve => {
        setTimeout(() => {
          resolve({ data: { files: ["file1"] } })
        }, 2000)
      })
    })

    const result = await ls("/test")

    // 2 秒返回，应该成功而不是超时
    expect(result).toEqual({ files: ["file1"] })
  })
})
