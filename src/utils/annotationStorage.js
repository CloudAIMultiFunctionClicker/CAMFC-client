// 标注数据存储工具 - 用 localStorage 保存标注数据
// 之前想过用 IndexedDB，但 localStorage 够用了，而且简单

const STORAGE_KEY = 'annotation_data'
const MAX_ITEMS = 50 // 最多保留 50 条，避免爆存储

/**
 * 保存标注数据
 * @param {string} imageId - 图片 ID
 * @param {Array} annotations - 标注数组
 * @param {string} imageData - 图片数据
 */
export const saveAnnotations = (imageId, annotations, imageData) => {
  try {
    const data = {
      imageId,
      annotations,
      imageData,
      timestamp: Date.now()
    }
    
    const existingData = getAllAnnotations()
    const index = existingData.findIndex(item => item.imageId === imageId)
    
    if (index !== -1) {
      existingData[index] = data
    } else {
      existingData.push(data)
    }
    
    // 限制数量，保留最新的
    const limitedData = existingData.slice(-MAX_ITEMS)
    localStorage.setItem(STORAGE_KEY, JSON.stringify(limitedData))
    return true
  } catch (error) {
    console.error('保存标注失败:', error)
    return false
  }
}

/**
 * 加载标注数据
 * @param {string} imageId - 图片 ID
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
    console.error('加载标注失败:', error)
    return null
  }
}

/**
 * 获取所有标注
 */
export const getAllAnnotations = () => {
  try {
    const data = localStorage.getItem(STORAGE_KEY)
    return data ? JSON.parse(data) : []
  } catch (error) {
    console.error('读取标注失败:', error)
    return []
  }
}

/**
 * 删除指定图片的标注
 * @param {string} imageId - 图片 ID
 */
export const deleteAnnotations = (imageId) => {
  try {
    const data = getAllAnnotations()
    const filteredData = data.filter(item => item.imageId !== imageId)
    localStorage.setItem(STORAGE_KEY, JSON.stringify(filteredData))
    return true
  } catch (error) {
    console.error('删除标注失败:', error)
    return false
  }
}

/**
 * 清空所有标注
 */
export const clearAllAnnotations = () => {
  try {
    localStorage.removeItem(STORAGE_KEY)
    return true
  } catch (error) {
    console.error('清空标注失败:', error)
    return false
  }
}

/**
 * 生成图片 ID（简单哈希）
 * @param {string} imageData - 图片数据
 */
export const generateImageId = (imageData) => {
  // 只取前 1000 字符计算，避免太长
  let hash = 0
  const str = imageData.substring(0, 1000)
  
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash
  }
  
  return `img_${Math.abs(hash)}_${Date.now()}`
}
