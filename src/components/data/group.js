/*
保留所有权利

Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh09@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

Copyright (C) 2026 Kaibin Zeng (曾楷彬) <https://github.com/Waple1145>
Email: admin@mc666.top
*/

import axios from "axios";
import { getBackendUrl } from "../../config/backend.js";

const timeOut = 3000;

// 获取认证头信息（和 fileSystem.js 保持一致）
async function getAuthHeader() {
  try {
    const { getDeviceId, getTotp } = await import('./bluetooth.js');
    
    const deviceId = await getDeviceId();
    const currentTotp = await getTotp();
    
    console.info({
      "Id": deviceId,
      "Totp": currentTotp
    });

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
 * 创建群组
 * @param {string} name - 群组名称（1-15 字符）
 * @returns {Promise<Object|null>} - 成功返回 {uid: "xxx"}，失败返回 null
 */
async function createGroup(name) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();
    
    const requestPromise = axios.post(
      getBackendUrl() + "/group/create",
      { name: name },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('创建群组成功:', response.data);
    
    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('创建群组失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

/**
 * 删除群组
 * @param {string} uid - 群组 UID
 * @returns {Promise<Object|null>} - 成功返回 {success: true}，失败返回 null
 */
async function deleteGroup(uid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();
    
    const requestPromise = axios.post(
      getBackendUrl() + "/group/delete",
      { uid: uid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('删除群组成功:', response.data);
    
    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('删除群组失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

/**
 * 查询消息详情
 * @param {string} uuid - 消息 UUID
 * @returns {Promise<Object|null>} - 返回消息详情，失败返回 null
 */
async function queryMessage(uuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();
    
    const requestPromise = axios.post(
      getBackendUrl() + "/group/query_message",
      { uuid: uuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('查询消息成功:', response.data);
    
    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('查询消息失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

/**
 * 批准入群/退群申请
 * @param {string} uuid - 消息 UUID
 * @returns {Promise<Object|null>} - 成功返回 {success: true}，失败返回 null
 */
async function allowApplication(uuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();
    
    const requestPromise = axios.post(
      getBackendUrl() + "/group/allow",
      { uuid: uuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('批准申请成功:', response.data);
    
    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('批准申请失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

/**
 * 获取群组列表
 * @returns {Promise<Array>} - 返回群组列表
 */
async function getGroupList() {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();
    
    const requestPromise = axios.get(
      getBackendUrl() + "/group/list",
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取群组列表:', response.data);
    
    // 后端返回格式：{ groups: [...] }
    return response.data?.groups || [];
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return [];
    } else {
      // 后端可能还没实现这个接口，静默处理
      console.warn('获取群组列表失败（后端可能未实现）:', error.response?.data?.detail || error.message);
      return [];
    }
  }
}

/**
 * 获取消息列表
 * @returns {Promise<Array>} - 返回消息列表
 */
async function getMessageList() {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();
    
    const requestPromise = axios.get(
      getBackendUrl() + "/group/messages",
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取消息列表:', response.data);
    
    // 后端返回格式：{ messages: [...] }
    return response.data?.messages || [];
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return [];
    } else {
      console.error('获取消息列表失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

export { createGroup, deleteGroup, queryMessage, allowApplication, getGroupList, getMessageList };
