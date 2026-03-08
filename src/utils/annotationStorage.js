/**
 * 标注数据存储工具
 * 用于保存和加载标注数据
 */

const STORAGE_KEY = 'annotation_data'

/**
 * 保存标注数据
 * @param {string} imageId - 图片唯一标识
 * @param {Array} annotations - 标注数据数组
 * @param {string} imageData - 标注后的图片数据
 * @returns {boolean} 保存是否成功
 */
export const saveAnnotations = (imageId, annotations, imageData) => {
  try {
    const data = {
      imageId,
      annotations,
      imageData,
      timestamp: Date.now()
    }
    
    // 获取现有数据
    const existingData = getAllAnnotations()
    
    // 查找是否已存在该图片的标注
    const index = existingData.findIndex(item => item.imageId === imageId)
    
    if (index !== -1) {
      // 更新现有标注
      existingData[index] = data
    } else {
      // 添加新标注
      existingData.push(data)
    }
    
    // 限制存储数量，保留最近的 50 条
    const limitedData = existingData.slice(-50)
    
    localStorage.setItem(STORAGE_KEY, JSON.stringify(limitedData))
    return true
  } catch (error) {
    console.error('保存标注数据失败:', error)
    return false
  }
}

/**
 * 加载指定图片的标注数据
 * @param {string} imageId - 图片唯一标识
 * @returns {Object|null} 标注数据对象，包含 annotations 和 imageData
 */
export const loadAnnotations = (imageId) => {
  try {
    const data = getAllAnnotations()
    const item = data.find(item => item.imageId === imageId)
    
    if (item) {
      return {
        annotations: item.annotations,
        imageData: item.imageData,
        timestamp: item.timestamp
      }
    }
    
    return null
  } catch (error) {
    console.error('加载标注数据失败:', error)
    return null
  }
}

/**
 * 获取所有标注数据
 * @returns {Array} 所有标注数据数组
 */
export const getAllAnnotations = () => {
  try {
    const data = localStorage.getItem(STORAGE_KEY)
    return data ? JSON.parse(data) : []
  } catch (error) {
    console.error('获取所有标注数据失败:', error)
    return []
  }
}

/**
 * 删除指定图片的标注数据
 * @param {string} imageId - 图片唯一标识
 * @returns {boolean} 删除是否成功
 */
export const deleteAnnotations = (imageId) => {
  try {
    const data = getAllAnnotations()
    const filteredData = data.filter(item => item.imageId !== imageId)
    localStorage.setItem(STORAGE_KEY, JSON.stringify(filteredData))
    return true
  } catch (error) {
    console.error('删除标注数据失败:', error)
    return false
  }
}

/**
 * 清空所有标注数据
 * @returns {boolean} 清空是否成功
 */
export const clearAllAnnotations = () => {
  try {
    localStorage.removeItem(STORAGE_KEY)
    return true
  } catch (error) {
    console.error('清空标注数据失败:', error)
    return false
  }
}

/**
 * 生成图片唯一标识
 * @param {string} imageData - 图片数据
 * @returns {string} 图片唯一标识
 */
export const generateImageId = (imageData) => {
  // 使用简单的哈希算法生成唯一标识
  let hash = 0
  const str = imageData.substring(0, 1000) // 只取前 1000 个字符计算哈希
  
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash // Convert to 32bit integer
  }
  
  return `img_${Math.abs(hash)}_${Date.now()}`
}
