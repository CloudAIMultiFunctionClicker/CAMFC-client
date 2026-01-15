/*
<!--
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

/**
 * 使用 RSA-OAEP 加密文本（公钥为 PEM 格式）
 * @param {string} pemPublicKey - PEM 格式的 RSA 公钥（带 -----BEGIN PUBLIC KEY----- 头尾）
 * @param {string} text - 要加密的明文
 * @returns {Promise<string>} - 返回 Base64 编码的加密结果
 */
async function encrypt(pemPublicKey, text) {
    // 1. 将 PEM 公钥转换为 ArrayBuffer（去除头尾和换行）
    const pem = pemPublicKey
        .replace(/\r?\n|\r/g, "")
        .replace("-----BEGIN PUBLIC KEY-----", "")
        .replace("-----END PUBLIC KEY-----", "");

    const binaryDerString = atob(pem);
    const binaryDer = new Uint8Array(binaryDerString.length);
    for (let i = 0; i < binaryDerString.length; i++) {
        binaryDer[i] = binaryDerString.charCodeAt(i);
    }

    // 2. 导入公钥
    const publicKey = await window.crypto.subtle.importKey(
        "spki",
        binaryDer,
        {
            name: "RSA-OAEP",
            hash: "SHA-256",
        },
        true,
        ["encrypt"]
    );

    // 3. 编码明文为 Uint8Array
    const encoder = new TextEncoder();
    const data = encoder.encode(text);

    // 4. 执行加密
    const encrypted = await window.crypto.subtle.encrypt(
        {
            name: "RSA-OAEP",
        },
        publicKey,
        data
    );

    // 5. 转为 Base64 字符串返回
    const encryptedArray = new Uint8Array(encrypted);
    let base64String = "";
    const chunkSize = 0x4000; // 避免长字符串导致内存问题
    for (let i = 0; i < encryptedArray.length; i += chunkSize) {
        base64String += String.fromCharCode.apply(
            null,
            encryptedArray.subarray(i, i + chunkSize)
        );
    }
    console.log('encrypted')
    console.log(btoa(base64String))
    return btoa(base64String);

}

export { encrypt };
