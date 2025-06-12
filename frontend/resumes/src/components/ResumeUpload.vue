<script setup lang="ts">
import { ref, computed } from 'vue'

// 定义事件
const emit = defineEmits<{
  'file-uploaded': [file: File]
  'upload-error': [error: string]
}>()

// 响应式数据
const isDragging = ref(false)
const isUploading = ref(false)
const uploadProgress = ref(0)
const uploadedFile = ref<File | null>(null)
const errorMessage = ref('')

// 允许的文件类型
const allowedTypes = ['.pdf', '.doc', '.docx']
const maxFileSize = 10 * 1024 * 1024 // 10MB

// 计算属性
const isFileSelected = computed(() => uploadedFile.value !== null)

// 验证文件
const validateFile = (file: File): string | null => {
  // 检查文件大小
  if (file.size > maxFileSize) {
    return `文件大小不能超过 ${maxFileSize / 1024 / 1024}MB`
  }
  
  // 检查文件类型
  const fileExtension = '.' + file.name.split('.').pop()?.toLowerCase()
  if (!allowedTypes.includes(fileExtension)) {
    return `只支持以下格式: ${allowedTypes.join(', ')}`
  }
  
  return null
}

// 处理文件上传
const handleFileUpload = async (file: File) => {
  const validationError = validateFile(file)
  if (validationError) {
    errorMessage.value = validationError
    emit('upload-error', validationError)
    return
  }
  
  errorMessage.value = ''
  isUploading.value = true
  uploadProgress.value = 0
  
  try {
    // 模拟上传进度
    const progressInterval = setInterval(() => {
      uploadProgress.value += 10
      if (uploadProgress.value >= 100) {
        clearInterval(progressInterval)
      }
    }, 100)
    
    // 等待模拟上传完成
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    uploadedFile.value = file
    emit('file-uploaded', file)
    console.log('文件上传成功:', file.name)
  } catch (error) {
    const errorMsg = '上传失败，请重试'
    errorMessage.value = errorMsg
    emit('upload-error', errorMsg)
  } finally {
    isUploading.value = false
    uploadProgress.value = 0
  }
}

// 文件选择事件
const onFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    handleFileUpload(file)
  }
}

// 拖拽事件处理
const onDragEnter = (e: DragEvent) => {
  e.preventDefault()
  isDragging.value = true
}

const onDragLeave = (e: DragEvent) => {
  e.preventDefault()
  if (!e.relatedTarget || !(e.currentTarget as Element)?.contains(e.relatedTarget as Node)) {
    isDragging.value = false
  }
}

const onDragOver = (e: DragEvent) => {
  e.preventDefault()
}

const onDrop = (e: DragEvent) => {
  e.preventDefault()
  isDragging.value = false
  
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    handleFileUpload(files[0])
  }
}

// 移除已上传的文件
const removeFile = () => {
  uploadedFile.value = null
  errorMessage.value = ''
}

// 格式化文件大小
const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>

<template>
  <div class="resume-upload">
    <!-- 上传区域 -->
    <div 
      class="upload-area"
      :class="{ 
        'dragging': isDragging, 
        'uploading': isUploading,
        'has-file': isFileSelected 
      }"
      @dragenter="onDragEnter"
      @dragleave="onDragLeave"
      @dragover="onDragOver"
      @drop="onDrop"
    >
      <input
        type="file"
        id="file-input"
        class="file-input"
        :accept="allowedTypes.join(',')"
        @change="onFileSelect"
      />
      
      <!-- 未选择文件时的界面 -->
      <div v-if="!isFileSelected && !isUploading" class="upload-prompt">
        <div class="upload-icon">📄</div>
        <h3>上传简历</h3>
        <p>拖拽文件到此处或 
          <label for="file-input" class="upload-link">点击选择文件</label>
        </p>
        <p class="file-info">
          支持格式: {{ allowedTypes.join(', ') }}<br>
          最大文件大小: {{ maxFileSize / 1024 / 1024 }}MB
        </p>
      </div>
      
      <!-- 上传进度 -->
      <div v-if="isUploading" class="upload-progress">
        <div class="progress-icon">⏳</div>
        <p>正在上传...</p>
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: uploadProgress + '%' }"></div>
        </div>
        <p class="progress-text">{{ uploadProgress }}%</p>
      </div>
      
      <!-- 已上传文件 -->
      <div v-if="isFileSelected && !isUploading" class="uploaded-file">
        <div class="file-icon">✅</div>
        <div class="file-details">
          <h4>{{ uploadedFile!.name }}</h4>
          <p>{{ formatFileSize(uploadedFile!.size) }}</p>
        </div>
        <button class="remove-button" @click="removeFile" title="移除文件">
          ✕
        </button>
      </div>
    </div>
    
    <!-- 错误信息 -->
    <div v-if="errorMessage" class="error-message">
      <span class="error-icon">⚠️</span>
      {{ errorMessage }}
    </div>
  </div>
</template>

<style scoped>
.resume-upload {
  position: relative;
}

.upload-area {
  border: 2px dashed #ddd;
  border-radius: 12px;
  padding: 2rem;
  text-align: center;
  transition: all 0.3s ease;
  background-color: #fafafa;
  cursor: pointer;
  min-height: 150px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.upload-area:hover {
  border-color: #42b883;
  background-color: rgba(66, 184, 131, 0.05);
}

.upload-area.dragging {
  border-color: #42b883;
  background-color: rgba(66, 184, 131, 0.1);
  transform: scale(1.02);
}

.upload-area.uploading {
  border-color: #42b883;
  background-color: rgba(66, 184, 131, 0.05);
}

.upload-area.has-file {
  border-color: #4caf50;
  background-color: rgba(76, 175, 80, 0.05);
}

.file-input {
  display: none;
}

.upload-prompt {
  width: 100%;
}

.upload-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.upload-prompt h3 {
  margin: 0.5rem 0;
  color: #333;
  font-weight: 600;
}

.upload-prompt p {
  margin: 0.5rem 0;
  color: #666;
}

.upload-link {
  color: #42b883;
  cursor: pointer;
  text-decoration: underline;
  font-weight: 500;
}

.upload-link:hover {
  color: #369870;
}

.file-info {
  font-size: 0.85rem;
  color: #888;
  margin-top: 1rem;
}

.upload-progress {
  width: 100%;
  max-width: 300px;
}

.progress-icon {
  font-size: 2rem;
  margin-bottom: 1rem;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: #e0e0e0;
  border-radius: 4px;
  overflow: hidden;
  margin: 1rem 0;
}

.progress-fill {
  height: 100%;
  background-color: #42b883;
  transition: width 0.3s ease;
}

.progress-text {
  font-weight: 600;
  color: #42b883;
}

.uploaded-file {
  display: flex;
  align-items: center;
  gap: 1rem;
  width: 100%;
  max-width: 400px;
  background-color: white;
  padding: 1rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.file-icon {
  font-size: 2rem;
  color: #4caf50;
}

.file-details {
  flex-grow: 1;
  text-align: left;
}

.file-details h4 {
  margin: 0;
  color: #333;
  font-size: 1rem;
  font-weight: 600;
}

.file-details p {
  margin: 0.25rem 0 0 0;
  color: #666;
  font-size: 0.9rem;
}

.remove-button {
  background: none;
  border: none;
  color: #f44336;
  font-size: 1.2rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.remove-button:hover {
  background-color: rgba(244, 67, 54, 0.1);
  transform: scale(1.1);
}

.error-message {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #f44336;
  background-color: rgba(244, 67, 54, 0.1);
  padding: 0.75rem 1rem;
  border-radius: 6px;
  margin-top: 1rem;
  font-size: 0.9rem;
}

.error-icon {
  font-size: 1.1rem;
}

@media (prefers-color-scheme: dark) {
  .upload-area {
    background-color: #333;
    border-color: #555;
  }
  
  .upload-area:hover {
    background-color: rgba(66, 184, 131, 0.1);
  }
  
  .upload-prompt h3 {
    color: #e9ecef;
  }
  
  .upload-prompt p {
    color: #adb5bd;
  }
  
  .file-info {
    color: #6c757d;
  }
  
  .uploaded-file {
    background-color: #444;
  }
  
  .file-details h4 {
    color: #e9ecef;
  }
  
  .file-details p {
    color: #adb5bd;
  }
  
  .error-message {
    background-color: rgba(244, 67, 54, 0.2);
  }
}

@media (max-width: 768px) {
  .upload-area {
    padding: 1.5rem 1rem;
    min-height: 120px;
  }
  
  .upload-icon {
    font-size: 2.5rem;
  }
  
  .uploaded-file {
    flex-direction: column;
    text-align: center;
    gap: 0.5rem;
  }
  
  .file-details {
    text-align: center;
  }
}
</style>