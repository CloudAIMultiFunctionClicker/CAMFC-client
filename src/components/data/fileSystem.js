

import axios from "axios";
import { ref, reactive } from "vue";
import { getBackendUrl } from "../../config/backend.js";

const timeOut = 3000;

async function getAuthHeader() {
  try {

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
    console.warn('无法获取设备ID或TOTP，使用空header:', error);

    return {};
  }
}

async function ls(path) {
    try {

        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error("Request timeout"));
            }, timeOut);
        });

        const authHeader = await getAuthHeader();

        const requestPromise = axios.get(getBackendUrl() + "/files/?path=" + path, {
            headers: authHeader,
        });

        const response = await Promise.race([requestPromise, timeoutPromise]);
        console.log(response.data);

        return response.data;
    } catch (error) {
        if (error.message === "Request timeout") {
            console.warn(`Request timed out after ${timeOut}ms`);

            return null;
        } else {
            throw error;
        }
    }
}

async function mkdir(path, directoryName) {
    try {

        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error("Request timeout"));
            }, timeOut);
        });

        const authHeader = await getAuthHeader();

        const params = new URLSearchParams({
            path: path,
            directory_name: directoryName,
        });

        const requestPromise = axios.post(
            getBackendUrl() + "/files/directories",
            null,
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
            throw error;
        }
    }
}

async function rm(path, permanent = false) {
    try {

        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => {
                reject(new Error("Request timeout"));
            }, timeOut);
        });

        const authHeader = await getAuthHeader();

        const params = new URLSearchParams({
            permanent: permanent.toString(),
        });

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
            throw error;
        }
    }
}
export {ls,mkdir,rm};
