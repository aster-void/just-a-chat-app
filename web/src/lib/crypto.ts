export async function hashPassword(password: string): Promise<string> {
  return await sha256(password);
}

export async function sha256(str: string): Promise<string> {
  // encode as UTF-8
  const msgBuffer = new TextEncoder().encode(str);

  // hash the message
  const hashBuffer = await crypto.subtle.digest("SHA-256", msgBuffer);

  // convert ArrayBuffer to Array
  const hashArray = Array.from(new Uint8Array(hashBuffer));

  // convert bytes to hex string
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
  return hashHex;
}
