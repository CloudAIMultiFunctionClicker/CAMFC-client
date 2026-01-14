/**
 * 弹出提示卡片（Toast）
 * TODO: 可以加个队列功能，防止多个toast同时出现时重叠
 * 
 * @param {string} text - 显示的文字
 * @param {string} [color='#4f46e5'] - 背景色，默认是紫色，还挺好看的
 * @param {number} [duration=3000] - 多久自动消失（毫秒），0表示不自动消失
 */
function showToast(text, color = "#4f46e5", duration = 3000) {
    // 创建容器（如果还没有的话）
    let toastContainer = document.getElementById("toast-container");
    if (!toastContainer) {
        toastContainer = document.createElement("div");
        toastContainer.id = "toast-container";
        // 这里用CSS变量会更灵活，但为了简单先用固定样式
        // 不知道项目里有没有全局CSS变量，先写死吧
        toastContainer.style.cssText = `
      position: fixed;
      top: 20px;
      right: 20px;
      z-index: 10000;
      display: flex;
      flex-direction: column;
      gap: 10px;
      pointer-events: none;
    `;
        document.body.appendChild(toastContainer);
    }

    // 创建toast卡片
    const toast = document.createElement("div");
    toast.textContent = text;
    toast.style.cssText = `
    background: ${color};
    color: white;
    padding: 12px 20px;
    border-radius: 12px;
    box-shadow: 0 6px 20px rgba(0,0,0,0.15);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    font-size: 14px;
    font-weight: 500;
    max-width: 320px;
    word-break: break-word;
    animation: toast-slide-in 0.3s ease forwards,
               toast-fade-out 0.4s ease forwards ${duration > 0 ? duration : "999999"}ms;
    pointer-events: auto;
    opacity: 0;
  `;

    // 动画样式（只加一次，不然会重复添加）
    // 这里用个id来避免重复添加，不知道有没有更好的方法
    if (!document.getElementById("toast-styles")) {
        const style = document.createElement("style");
        style.id = "toast-styles";
        style.textContent = `
      @keyframes toast-slide-in {
        from {
          transform: translateX(120%) scale(0.95);
          opacity: 0;
        }
        to {
          transform: translateX(0) scale(1);
          opacity: 1;
        }
      }
      @keyframes toast-fade-out {
        from {
          opacity: 1;
          transform: translateX(0) scale(1);
        }
        to {
          opacity: 0;
          transform: translateX(120%) scale(0.9);
        }
      }
    `;
        document.head.appendChild(style);
    }

    // 加到容器里
    toastContainer.appendChild(toast);

    // 自动消失（如果设置了duration）
    if (duration > 0) {
        setTimeout(() => {
            if (toast.parentNode) {
                toast.style.animation = "toast-fade-out 0.4s ease forwards";
                // 等动画播完再移除
                setTimeout(() => {
                    toast.remove();
                }, 400);
            }
        }, duration);
    }

    // 点击就关闭，这个功能挺好用的
    toast.addEventListener("click", () => {
        toast.style.animation = "toast-fade-out 0.3s ease forwards";
        setTimeout(() => toast.remove(), 300);
    });
}

export { showToast };
