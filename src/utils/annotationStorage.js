

const STORAGE_KEY = 'annotation_data'

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

    const limitedData = existingData.slice(-50)

    localStorage.setItem(STORAGE_KEY, JSON.stringify(limitedData))
    return true
  } catch (error) {
    console.error('保存标注数据失败:', error)
    return false
  }
}

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

export const getAllAnnotations = () => {
  try {
    const data = localStorage.getItem(STORAGE_KEY)
    return data ? JSON.parse(data) : []
  } catch (error) {
    console.error('获取所有标注数据失败:', error)
    return []
  }
}

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

export const clearAllAnnotations = () => {
  try {
    localStorage.removeItem(STORAGE_KEY)
    return true
  } catch (error) {
    console.error('清空标注数据失败:', error)
    return false
  }
}

export const generateImageId = (imageData) => {

  let hash = 0
  const str = imageData.substring(0, 1000)

  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash
  }

  return `img_${Math.abs(hash)}_${Date.now()}`
}
