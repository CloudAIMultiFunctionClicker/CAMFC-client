/**
 * 示例测试文件
 * 演示如何编写 Vitest 测试
 */
import { describe, it, expect } from "vitest";

// 简单的工具函数测试示例
function add(a, b) {
  return a + b;
}

describe("基础测试示例", () => {
  it("应该能正确执行加法运算", () => {
    expect(add(1, 2)).toBe(3);
    expect(add(-1, 1)).toBe(0);
  });

  it("应该能比较字符串", () => {
    expect("hello").toBe("hello");
    expect("world").not.toBe("Hello");
  });
});

// Vue 组件测试示例（使用 @vue/test-utils）
describe("Vue 组件测试示例", () => {
  it("演示如何测试 Vue 组件", async () => {
    // TODO: 实际使用时需要安装 @vue/test-utils
    // 示例代码：
    // const wrapper = mount(YourComponent, {
    //   props: { someProp: 'value' }
    // })
    // expect(wrapper.text()).toContain('expected text')
    
    // 这里只是演示结构，实际测试需要安装依赖后再写
    expect(true).toBe(true);
  });
});
