
*/

async function encrypt(pemPublicKey, text) {

    const pem = pemPublicKey
        .replace(/\r?\n|\r/g, "")
        .replace("-----BEGIN PUBLIC KEY-----", "")
        .replace("-----END PUBLIC KEY-----", "");

    const binaryDerString = atob(pem);
    const binaryDer = new Uint8Array(binaryDerString.length);
    for (let i = 0; i < binaryDerString.length; i++) {
        binaryDer[i] = binaryDerString.charCodeAt(i);
    }

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

    const encoder = new TextEncoder();
    const data = encoder.encode(text);

    const encrypted = await window.crypto.subtle.encrypt(
        {
            name: "RSA-OAEP",
        },
        publicKey,
        data
    );

    const encryptedArray = new Uint8Array(encrypted);
    let base64String = "";
    const chunkSize = 0x4000;
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
