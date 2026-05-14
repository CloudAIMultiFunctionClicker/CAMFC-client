

import axios from "axios";
import { getBackendUrl } from "../../config/backend.js";

const timeOut = 3000;

async function getAuthHeader(retryCount = 0) {
  const maxRetries = 3;
  const retryDelay = 300;

  try {
    const { getDeviceId, getTotp } = await import('./bluetooth.js');

    const deviceId = await getDeviceId();
    const currentTotp = await getTotp();

    if (currentTotp && deviceId) {
      console.info('使用教师认证:', {
        "Id": deviceId,
        "Totp": currentTotp
      });

      return {
        "Id": deviceId,
        "Totp": currentTotp
      };
    }

    if (retryCount < maxRetries) {
      console.log(`等待蓝牙模块准备中... (${retryCount + 1}/${maxRetries})`);
      await new Promise(resolve => setTimeout(resolve, retryDelay));
      return await getAuthHeader(retryCount + 1);
    }
  } catch (error) {
    console.log('无法获取蓝牙设备信息，尝试学生认证');
  }

  try {
    const { loadAppData } = await import('./storage.js');
    const studentUsername = await loadAppData('student_username');
    const studentPassword = await loadAppData('student_password');

    if (studentUsername && studentPassword) {
      console.info('使用学生认证:', {
        "Username": studentUsername

      });

      return {
        "Username": studentUsername,
        "Password": studentPassword
      };
    }
  } catch (error) {
    console.warn('无法获取学生认证信息:', error);
  }

  console.warn('无法获取任何认证信息，使用空 header');
  return {};
}

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

    return response.data?.groups || [];
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return [];
    } else {

      console.warn('获取群组列表失败（后端可能未实现）:', error.response?.data?.detail || error.message);
      return [];
    }
  }
}

async function getMessageList() {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.get(
      getBackendUrl() + "/group/messages?status=pending",
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取消息列表:', response.data);

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

async function approveJoin(uuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + "/group/approve_join",
      { message_uuid: uuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('批准加入申请成功:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('批准加入申请失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function rejectJoin(uuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + "/group/reject_join",
      { message_uuid: uuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('拒绝加入申请成功:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('拒绝加入申请失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function approveQuit(uuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + "/group/approve_quit",
      { message_uuid: uuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('批准退出申请成功:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('批准退出申请失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function shareNoteToGroup(noteUuid, groupUuid, noteType, meetingUuid = null) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestBody = {
      note_uuid: noteUuid,
      group_uuid: groupUuid,
      note_type: noteType
    };

    if (noteType === "meeting" && meetingUuid) {
      requestBody.meeting_uuid = meetingUuid;
    }

    const requestPromise = axios.post(
      getBackendUrl() + "/group/share/note",
      requestBody,
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('分享笔记到群组成功:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('分享笔记到群组失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getSharedNotes(groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.get(
      getBackendUrl() + `/group/share/notes?group_uuid=${groupUuid}`,
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取群组共享笔记列表:', response.data);

    return response.data?.notes || [];
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return [];
    } else {
      console.error('获取群组共享笔记列表失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getSharedNoteDetail(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + "/group/share/note/detail",
      { share_uuid: shareUuid, group_uuid: groupUuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取共享笔记详情:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('获取共享笔记详情失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getNoteInteractions(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.get(
      getBackendUrl() + `/student/note-interactions/teacher/all?group_uid=${groupUuid}&share_uuid=${shareUuid}`,
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取笔记互动数据:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('获取笔记互动数据失败:', error.response?.data || error.message);
      return null;
    }
  }
}

async function recordNoteRead(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + '/student/note-interactions/read',
      { share_uuid: shareUuid, group_uid: groupUuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('记录阅读成功:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('记录阅读失败:', error.response?.data || error.message);
      return null;
    }
  }
}

async function shareFileToGroup(filePath, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + "/group/share/file",
      {
        file_path: filePath,
        group_uuid: groupUuid
      },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('分享文件到群组成功:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('分享文件到群组失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getSharedFiles(groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.get(
      getBackendUrl() + `/group/share/files?group_uuid=${groupUuid}`,
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取群组共享文件列表:', response.data);

    return response.data?.files || [];
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return [];
    } else {
      console.error('获取群组共享文件列表失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getSharedFileDetail(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + "/group/share/file/detail",
      { share_uuid: shareUuid, group_uuid: groupUuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取共享文件详情:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('获取共享文件详情失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getSharedFileDetailForTeacher(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.post(
      getBackendUrl() + "/group/share/file/detail",
      { share_uuid: shareUuid, group_uuid: groupUuid },
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取共享文件详情（教师端）:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('获取共享文件详情（教师端）失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getSharedFileDownloadInfo(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.get(
      getBackendUrl() + `/group/share/file/download?share_uuid=${shareUuid}&group_uuid=${groupUuid}`,
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取共享文件下载信息:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('获取共享文件下载信息失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function getSharedFileDownloadInfoForTeacher(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.get(
      getBackendUrl() + `/group/share/file/download?share_uuid=${shareUuid}&group_uuid=${groupUuid}`,
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('获取共享文件下载信息（教师端）:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('获取共享文件下载信息（教师端）失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

async function deleteSharedFile(shareUuid, groupUuid) {
  try {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error("Request timeout"));
      }, timeOut);
    });

    const authHeader = await getAuthHeader();

    const requestPromise = axios.delete(
      getBackendUrl() + `/group/share/file?share_uuid=${shareUuid}&group_uuid=${groupUuid}`,
      {
        headers: authHeader,
      }
    );

    const response = await Promise.race([requestPromise, timeoutPromise]);
    console.info('删除共享文件:', response.data);

    return response.data;
  } catch (error) {
    if (error.message === "Request timeout") {
      console.warn(`请求超时 (${timeOut}ms)`);
      return null;
    } else {
      console.error('删除共享文件失败:', error.response?.data || error.message);
      throw error;
    }
  }
}

export {
  createGroup,
  deleteGroup,
  queryMessage,
  allowApplication,
  getGroupList,
  getMessageList,
  approveJoin,
  rejectJoin,
  approveQuit,
  shareNoteToGroup,
  getSharedNotes,
  getSharedNoteDetail,
  getNoteInteractions,
  recordNoteRead,
  shareFileToGroup,
  getSharedFiles,
  getSharedFileDetail,
  getSharedFileDetailForTeacher,
  getSharedFileDownloadInfo,
  getSharedFileDownloadInfoForTeacher,
  deleteSharedFile,
  getAuthHeader
};
