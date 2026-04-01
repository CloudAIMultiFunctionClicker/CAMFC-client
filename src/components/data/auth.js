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

/**
 * 使用 RSA-OAEP 加密文本
 * @param {string} pemPublicKey - PEM 格式公钥（带 BEGIN/END 头尾）
 * @param {string} text - 要加密的明文
 * @returns {Promise<string>} Base64 加密结果
 */
async function encrypt(pemPublicKey, text) {
    // 解析 PEM 公钥，去掉头尾标记和换行
    const pem = pemPublicKey
        .replace(/\r?\n|\r/g, "")
        .replace("-----BEGIN PUBLIC KEY-----", "")
        .replace("-----END PUBLIC KEY-----", "");

    // Base64 解码为二进制数据
    const binaryDerString = atob(pem);
    const binaryDer = new Uint8Array(binaryDerString.length);
    for (let i = 0; i < binaryDerString.length; i++) {
        binaryDer[i] = binaryDerString.charCodeAt(i);
    }

    // 导入公钥到 Web Crypto API
    const publicKey = await window.crypto.subtle.importKey(
        "spki",
        binaryDer,
        { name: "RSA-OAEP", hash: "SHA-256" },
        true,
        ["encrypt"]
    );

    // 明文转 Uint8Array
    const encoder = new TextEncoder();
    const data = encoder.encode(text);

    // 执行 RSA-OAEP 加密
    const encrypted = await window.crypto.subtle.encrypt(
        { name: "RSA-OAEP" },
        publicKey,
        data
    );

    // 加密结果转 Base64（分块处理避免长字符串问题）
    const encryptedArray = new Uint8Array(encrypted);
    let base64String = "";
    const chunkSize = 0x4000;
    for (let i = 0; i < encryptedArray.length; i += chunkSize) {
        base64String += String.fromCharCode.apply(
            null,
            encryptedArray.subarray(i, i + chunkSize)
        );
    }

    return btoa(base64String);
}

export { encrypt };
