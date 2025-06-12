<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import ApplicantList from './components/ApplicantList.vue'
import ResumeUpload from './components/ResumeUpload.vue'

const showUploadModal = ref(false)

const openUploadModal = () => {
  showUploadModal.value = true
  document.body.classList.add('modal-open')
}

const closeUploadModal = () => {
  showUploadModal.value = false
  document.body.classList.remove('modal-open')
}

const handleFileUploaded = (file: File) => {
  console.log('文件上传成功:', file.name)
  // 这里可以添加文件处理逻辑
  // 例如解析简历内容并添加到应聘者列表
  closeUploadModal()
}

const handleUploadError = (error: string) => {
  console.error('上传错误:', error)
}

// 监听ESC键关闭模态框
const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && showUploadModal.value) {
    closeUploadModal()
  }
}

// 组件挂载时添加事件监听
onMounted(() => {
  document.addEventListener('keydown', handleKeyDown)
})

// 组件卸载时移除事件监听
onUnmounted(() => {
  document.removeEventListener('keydown', handleKeyDown)
  document.body.classList.remove('modal-open')
})
</script>

<template>
  <div class="header">
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
    <div class="upload-section">
      <button class="upload-button" @click="openUploadModal">
        <span>📄</span>
        上传简历
      </button>
    </div>
  </div>
  <main>
    <ApplicantList />
  </main>

  <!-- 上传简历模态框 -->
  <div v-if="showUploadModal" class="modal-overlay" @click="closeUploadModal">
    <div class="modal-container" @click.stop>
      <div class="modal-header">
        <h2>上传简历</h2>
        <button class="modal-close" @click="closeUploadModal">×</button>
      </div>
      <div class="modal-body">
        <ResumeUpload 
          @file-uploaded="handleFileUploaded"
          @upload-error="handleUploadError"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 2rem;
  padding: 0 1rem;
}

.upload-section {
  display: flex;
  align-items: center;
}

.upload-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: #42b883;
  color: white;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.3s ease;
  text-decoration: none;
  border: none;
  font-size: 1rem;
  font-family: inherit;
}

.upload-button:hover {
  background-color: #369870;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(66, 184, 131, 0.3);
}

.upload-button span {
  font-size: 1.2rem;
}

.logo {
  height: 4em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}

main {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
}

/* 模态框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.2s ease;
}

.modal-container {
  background-color: #fff;
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  animation: slideIn 0.3s ease;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  background-color: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: #2c3e50;
  font-weight: 600;
}

.modal-close {
  background: none;
  border: none;
  color: #6c757d;
  font-size: 1.8rem;
  cursor: pointer;
  padding: 0.25rem 0.5rem;
  border-radius: 50%;
  transition: all 0.2s ease;
  line-height: 1;
}

.modal-close:hover {
  background-color: rgba(108, 117, 125, 0.1);
  color: #2c3e50;
}

.modal-body {
  padding: 2rem;
  overflow-y: auto;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideIn {
  from { transform: translateY(-20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

/* 模态框打开时禁用背景滚动 */
:global(body.modal-open) {
  overflow: hidden;
}

@media (prefers-color-scheme: dark) {
  .modal-container {
    background-color: #2a2a2a;
  }
  
  .modal-header {
    background-color: #333;
    border-bottom-color: #444;
  }
  
  .modal-header h2 {
    color: #e9ecef;
  }
  
  .modal-close {
    color: #adb5bd;
  }
  
  .modal-close:hover {
    background-color: rgba(173, 181, 189, 0.1);
    color: #e9ecef;
  }
}

@media (max-width: 768px) {
  .modal-container {
    width: 95%;
    max-height: 95vh;
  }
  
  .modal-header {
    padding: 1rem;
  }
  
  .modal-header h2 {
    font-size: 1.25rem;
  }
  
  .modal-body {
    padding: 1.5rem;
  }
}
</style>
