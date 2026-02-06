/*<!--
Copyright (C) 2026 Jiale Xu (许嘉乐) (ANTmmmmm) <https://github.com/ant-cave>
Email: ANTmmmmm@outlook.com, ANTmmmmm@126.com, 1504596931@qq.com

Copyright (C) 2026 Xinhang Chen (陈欣航) <https://github.com/cxh09>
Email: abc.cxh2009@foxmail.com

Copyright (C) 2026 Zimo Wen (温子墨) <https://github.com/lusamaqq>
Email: 1220594170@qq.com

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
-->
*/

import axios from "axios";
import { ref, reactive } from "vue";
import { getBackendUrl } from "../../config/backend.js";

const timeOut = 3000;

// 获取当前认证头信息
// 直接从Rust命令获取设备ID和TOTP
async function getAuthHeader() {
  try {
    // 动态导入避免循环依赖
    const { getDeviceId, getTotp } = await import('./bluetooth.js');
    
    // 直接调用Rust命令获取实时数据
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
    console.warn('无法获取设备ID或TOTP，使用空header:', error);
    // 如果获取失败，返回空对象
    return {};
  }
}



async function ls(path) {
    try {
        // 创建一个Promise，用于控制超时
        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error("Request timeout"));
            }, timeOut);
        });

        // 获取认证头信息
        const authHeader = await getAuthHeader();
        
        // 使用Promise.race来竞争请求和超时
        const requestPromise = axios.get(getBackendUrl() + "/files/?path=" + path, {
            headers: authHeader,
        });

        const response = await Promise.race([requestPromise, timeoutPromise]);
        console.log(response.data);

        // 返回结果
        return response.data;
    } catch (error) {
        if (error.message === "Request timeout") {
            console.warn(`Request timed out after ${timeOut}ms`);

            //  返回 null
            return null;
        } else {
            throw error; // 如果不是超时错误，则重新抛出原始错误
        }
    }
}

/**
 * 创建新目录
 *
 * @param {string} path - 目录的父路径（相对于 storage 目录）
 * @param {string} directoryName - 要创建的目录名称（不能包含路径分隔符）
 * @returns {Promise<Object|null>} - 成功时返回响应数据，超时时返回 null，其他错误将抛出
 */
async function mkdir(path, directoryName) {
    try {
        // 创建超时 Promise
        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error("Request timeout"));
            }, timeOut);
        });

        // 获取认证头信息
        const authHeader = await getAuthHeader();
        
        // 构造查询参数
        const params = new URLSearchParams({
            path: path,
            directory_name: directoryName,
        });

        // 发起 POST 请求（axios 默认会将 params 附加到 URL 上）
        const requestPromise = axios.post(
            getBackendUrl() + "/files/directories",
            null, // 没有请求体，使用 null
            {
                params: params,
                headers: authHeader,
            }
        );

        const response = await Promise.race([requestPromise, timeoutPromise]);
        console.log("Directory created:", response.data);

        return response.data;
    } catch (error) {
        if (error.message === "Request timeout") {
            console.warn(`Request timed out after ${timeOut}ms`);
            return null;
        } else {
            throw error; // 重新抛出非超时错误
        }
    }
}

/**
 * 删除文件或目录
 *
 * @param {string} path - 要删除的文件或目录路径（相对于 storage 目录）
 * @param {boolean} [permanent=false] - 是否永久删除（不进入回收站）
 * @returns {Promise<Object|null>} - 成功时返回响应数据，超时时返回 null，其他错误将抛出
 */
async function rm(path, permanent = false) {
    try {
        // 创建超时 Promise
        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error("Request timeout"));
            }, timeOut);
        });

        // 获取认证头信息
        const authHeader = await getAuthHeader();
        
        // 构造查询参数
        const params = new URLSearchParams({
            permanent: permanent.toString(),
        });

        // 发起 DELETE 请求
        const requestPromise = axios.delete(
            `${getBackendUrl()}/files/${encodeURIComponent(path)}`,
            {
                params: params,
                headers: authHeader,
            }
        );

        const response = await Promise.race([requestPromise, timeoutPromise]);
        console.log("File or directory deleted:", response.data);

        return response.data;
    } catch (error) {
        if (error.message === "Request timeout") {
            console.warn(`Request timed out after ${timeOut}ms`);
            return null;
        } else {
            throw error; // 重新抛出非超时错误
        }
    }
}
export {ls,mkdir,rm};
