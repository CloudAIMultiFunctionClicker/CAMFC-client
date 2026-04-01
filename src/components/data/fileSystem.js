/*
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
*/

import axios from "axios";
import { ref, reactive } from "vue";
import { getBackendUrl } from "../../config/backend.js";

// 请求超时时间（毫秒）
const timeOut = 3000;

/**
 * 获取认证头信息
 * 从 bluetooth.js 动态导入并调用 Rust 命令获取设备 ID 和 TOTP
 * @returns {Promise<Object>} 包含 Id 和 Totp 的对象，失败时返回空对象
 */
async function getAuthHeader() {
  try {
    // 动态导入避免循环依赖
    const { getDeviceId, getTotp } = await import('./bluetooth.js');
    
    const deviceId = await getDeviceId();
    const currentTotp = await getTotp();
    
    console.info({
      "Id": deviceId,
      "Totp": currentTotp
    })

    return {
      "Id": deviceId,
      "Totp": currentTotp
    };
  } catch (error) {
    console.warn('无法获取设备 ID 或 TOTP，使用空 header:', error);
    return {};
  }
}

/**
 * 带超时控制的 API 请求封装
 * 统一处理超时和认证头获取
 * @param {Function} requestFn - 执行实际请求的函数
 * @returns {Promise<any|null>} 请求成功返回响应数据，超时返回 null
 */
async function requestWithTimeout(requestFn) {
  // 创建超时 Promise
  const timeoutPromise = new Promise((_, reject) => {
    setTimeout(() => {
      reject(new Error("Request timeout"));
    }, timeOut);
  });

  // 获取认证头
  const authHeader = await getAuthHeader();
  
  // 执行请求并与超时竞争
  const requestPromise = requestFn(authHeader);
  const response = await Promise.race([requestPromise, timeoutPromise]);
  
  return response.data;
}

/**
 * 列出指定路径下的文件和目录
 * @param {string} path - 要列出的目录路径（相对于 storage 目录）
 * @returns {Promise<Object|null>} 成功返回文件列表，超时返回 null，其他错误抛出
 */
async function ls(path) {
  try {
    return await requestWithTimeout((authHeader) => {
      return axios.get(getBackendUrl() + "/files/?path=" + path, {
        headers: authHeader,
      });
    });
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    }
    throw error;
  }
}

/**
 * 创建新目录
 * @param {string} path - 目录的父路径（相对于 storage 目录）
 * @param {string} directoryName - 要创建的目录名称（不能包含路径分隔符）
 * @returns {Promise<Object|null>} 成功返回响应数据，超时返回 null，其他错误抛出
 */
async function mkdir(path, directoryName) {
  try {
    return await requestWithTimeout((authHeader) => {
      const params = new URLSearchParams({
        path: path,
        directory_name: directoryName,
      });

      return axios.post(
        getBackendUrl() + "/files/directories",
        null,
        {
          params: params,
          headers: authHeader,
        }
      );
    });
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    }
    throw error;
  }
}

/**
 * 删除文件或目录
 * @param {string} path - 要删除的文件或目录路径（相对于 storage 目录）
 * @param {boolean} [permanent=false] - 是否永久删除（不进入回收站）
 * @returns {Promise<Object|null>} 成功返回响应数据，超时返回 null，其他错误抛出
 */
async function rm(path, permanent = false) {
  try {
    return await requestWithTimeout((authHeader) => {
      const params = new URLSearchParams({
        permanent: permanent.toString(),
      });

      return axios.delete(
        `${getBackendUrl()}/files/${encodeURIComponent(path)}`,
        {
          params: params,
          headers: authHeader,
        }
      );
    });
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    }
    throw error;
  }
}

export { ls, mkdir, rm };
