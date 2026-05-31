import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles/global.css";

// 测试渲染
console.log("main.tsx 开始执行");

const rootElement = document.getElementById("root");
if (!rootElement) {
  console.error("找不到 root 元素!");
} else {
  console.log("找到 root 元素，开始渲染...");
}

ReactDOM.createRoot(rootElement as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

console.log("React 渲染完成");