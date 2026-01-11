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

const basicUrl = "http://192.168.32.101:8005";
const timeOut = 1000;

async function ls(path) {
    try {
        // 创建一个Promise，用于控制超时
        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error("Request timeout"));
            }, timeOut);
        });

        // 使用Promise.race来竞争请求和超时
        const requestPromise = axios.get(basicUrl + "/files/?path=" + path, {
            headers: {
                Authorization: "Bearer test123",
            },
        });

        const response = await Promise.race([requestPromise, timeoutPromise]);
        console.log(response.data);
        return response.data;
    } catch (error) {
        if (error.message === "Request timeout") {
            console.warn(`Request timed out after ${timeOut}ms`);
            return null;
        } else {
            throw error; // 如果不是超时错误，则重新抛出原始错误
        }
    }
}

export default ls;
