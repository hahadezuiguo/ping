import { ElLoading } from 'element-plus'

const defaultOptions = {
  lock: true,
  text: '正在加载',
  background: 'rgba(0, 0, 0, 0.1)'
}
/**
 * 传入一个方法fn,在它执行周期内,加上全屏loading
 * 如果：
 * 1. fn是同步方法，结束后隐藏loading
 * 2. 如果是异步方法，resolve后隐藏loading
 * 3. 报错后隐藏loading并抛出错误
 * @param {*} fn 函数
 * @returns Function 一个新的函数，去执行它吧
 */
export const withLoading = (fn, options = {}) => {
  let loading;
  const showLoading = (options) => {
    loading = ElLoading.service(options)
  }

  const hideLoading = () => {
    if (loading) {
      loading.close()
    }
  }
  const _options = Object.assign(defaultOptions, options)
  const newFn = (...args) => {
    try {
      showLoading(_options)
      const result = fn(...args)
      const isPromise = result instanceof Promise
      if (!isPromise) {
        hideLoading()
        return result
      }
      return result
        .then((res) => {
          hideLoading()
          return res
        })
        .catch((err) => {
          hideLoading()
          throw err
        })
    } catch (err) {
      hideLoading()
      throw err
    }
  }
  return newFn
}

let loadingInstance = null; // 存储当前的加载实例
 
// 显示加载动画
export const showLoading = (options = {}) => {
  if (!loadingInstance) {
    loadingInstance = ElLoading.service({
      lock: true, // 锁定屏幕，防止用户在加载过程中进行其他操作
      text: options.text || '加载中...', // 自定义加载提示文本
      background: options.background || 'rgba(0, 0, 0, 0.7)', // 自定义背景颜色
      spinner: options.spinner || null, // 自定义加载动画（可选）
      fullscreen: true, // 全屏模式
    });
  }
};
 
// 隐藏加载动画
export const hideLoading = () => {
  if (loadingInstance) {
    loadingInstance.close(); // 关闭加载动画
    loadingInstance = null; // 重置实例
  }
};