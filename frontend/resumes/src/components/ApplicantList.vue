<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, provide } from 'vue';
import ApplicantCard from './ApplicantCard.vue';
import ApplicantScore from './ApplicantScore.vue';
import type { Applicant } from '../types/applicant';

// 控制当前打开的弹窗
const activePopupId = ref<number | null>(null);
// 追踪有弹窗打开的状态
const hasOpenPopup = ref(false);
// 当前选中的应聘者
const selectedApplicant = ref<Applicant | null>(null);

// 示例数据 - 在真实应用中，这些数据会从API或props获取
const applicants = ref<Applicant[]>([
  {
    id: 1,
    name: '张小明',
    scores: {
      experience: 8,
      education: 9,
      interview: 7,
      technical: 9,
      cultural: 8
    }
  },
  {
    id: 2,
    name: '李华',
    scores: {
      experience: 7,
      education: 6,
      interview: 8,
      technical: 7,
      cultural: 9
    }
  },
  {
    id: 3,
    name: '王芳',
    scores: {
      experience: 9,
      education: 7,
      interview: 9,
      technical: 8,
      cultural: 7
    }
  },
  {
    id: 4,
    name: '赵阳',
    scores: {
      experience: 6,
      education: 9,
      interview: 8,
      technical: 8,
      cultural: 7
    }
  },
  {
    id: 5,
    name: '陈明',
    scores: {
      experience: 9,
      education: 8,
      interview: 7,
      technical: 7,
      cultural: 8
    }
  },
  {
    id: 6,
    name: '林小雨',
    scores: {
      experience: 7,
      education: 8,
      interview: 7,
      technical: 8,
      cultural: 9
    }
  }
]);

// 搜索关键词
const searchQuery = ref('');

// 根据搜索过滤后的应聘者
const filteredApplicants = computed(() => {
  if (!searchQuery.value.trim()) return applicants.value;
  
  const query = searchQuery.value.toLowerCase().trim();
  return applicants.value.filter(applicant => 
    applicant.name.toLowerCase().includes(query)
  );
});

// 添加方法按平均分数排序应聘者
const sortByScore = () => {
  applicants.value.sort((a, b) => {
    const aScore = Object.values(a.scores).reduce((sum, score) => sum + score, 0) / 5;
    const bScore = Object.values(b.scores).reduce((sum, score) => sum + score, 0) / 5;
    return bScore - aScore; // 降序排列
  });
};

// 打开应聘者详情弹窗
const openApplicantDetails = (applicant: Applicant) => {
  selectedApplicant.value = applicant;
  activePopupId.value = applicant.id;
  hasOpenPopup.value = true;
  
  // 添加body类控制滚动
  document.body.classList.add('popup-open');
};

// 关闭弹窗
const closePopup = () => {
  activePopupId.value = null;
  hasOpenPopup.value = false;
  selectedApplicant.value = null;
  
  // 移除body类恢复滚动
  document.body.classList.remove('popup-open');
};

// 监听ESC键全局关闭弹窗
const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && hasOpenPopup.value) {
    closePopup();
  }
};

// 提供给子组件的弹窗控制方法
provide('openApplicantDetails', openApplicantDetails);

// 组件挂载时添加事件监听
onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

// 组件卸载时移除事件监听
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
  document.body.classList.remove('popup-open');
});

// 点击弹窗背景关闭弹窗
const onBackdropClick = (e: MouseEvent) => {
  if (e.target === e.currentTarget) {
    closePopup();
  }
};

// 组件加载时排序
sortByScore();
</script>

<template>
  <div class="applicant-list">
    <h1>应聘者评估</h1>
    
    <div class="controls">
      <div class="search-container">
        <input 
          type="text" 
          v-model="searchQuery" 
          placeholder="搜索应聘者..." 
          class="search-input"
        />
        <button class="clear-button" v-if="searchQuery" @click="searchQuery = ''">×</button>
      </div>
      <button @click="sortByScore" class="sort-button">
        按分数排序
      </button>
    </div>
    
    <div v-if="filteredApplicants.length === 0" class="empty-state">
      未找到应聘者
    </div>
    
    <div v-else class="applicants">
      <ApplicantCard 
        v-for="applicant in filteredApplicants" 
        :key="applicant.id" 
        :applicant="applicant"
        @click="openApplicantDetails(applicant)"
      />
    </div>
    
    <!-- 弹窗 -->
    <div v-if="hasOpenPopup" class="modal-overlay" @click="onBackdropClick">
      <div class="modal-container" @click.stop>
        <div class="modal-header">
          <h2>{{ selectedApplicant?.name }} 的详细评估</h2>
          <button class="modal-close" @click="closePopup">×</button>
        </div>
        <div class="modal-body">
          <ApplicantScore v-if="selectedApplicant" :applicant="selectedApplicant" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.applicant-list {
  max-width: 1000px;
  margin: 0 auto;
  padding: 1rem;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
  color: #2c3e50;
}

.controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  gap: 1rem;
  flex-wrap: wrap;
}

.search-container {
  position: relative;
  flex-grow: 1;
  max-width: 400px;
}

.search-input {
  width: 100%;
  padding: 0.5rem 1rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: all 0.2s ease;
}

.search-input:focus {
  border-color: var(--primary-color, #42b883);
  outline: none;
  box-shadow: 0 0 0 2px rgba(66, 184, 131, 0.2);
}

.clear-button {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  color: #999;
  cursor: pointer;
  font-size: 1.2rem;
  padding: 0;
}

.clear-button:hover {
  color: #333;
}

.sort-button {
  background-color: var(--primary-color, #42b883);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: background-color 0.2s ease;
  white-space: nowrap;
}

.sort-button:hover {
  background-color: var(--secondary-color, #35495e);
}

.empty-state {
  text-align: center;
  padding: 3rem;
  background-color: #f8f9fa;
  border-radius: 8px;
  color: #6c757d;
  font-size: 1.2rem;
}

.applicants {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
  position: relative;
}

/* 弹窗打开时的样式 */
body.popup-open {
  overflow: hidden;
  padding-right: 15px; /* 防止滚动条消失导致的页面抖动 */
}

/* 弹窗样式 */
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
  background-color: #f8f9fa;
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 900px;
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
  padding: 1rem;
  background-color: #2c3e50;
  color: white;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.2rem;
}

.modal-close {
  background: none;
  border: none;
  color: white;
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  transition: all 0.2s;
}

.modal-close:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.modal-body {
  overflow-y: auto;
  max-height: calc(90vh - 60px);
  padding: 0;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideIn {
  from { transform: translateY(-20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

@media (prefers-color-scheme: dark) {
  h1 {
    color: #e9ecef;
  }
  
  .empty-state {
    background-color: #2a2a2a;
    color: #adb5bd;
  }
  
  .search-input {
    background-color: #333;
    border-color: #444;
    color: #eee;
  }
  
  .search-input:focus {
    border-color: var(--primary-color, #42b883);
  }
  
  .clear-button {
    color: #aaa;
  }
  
  .clear-button:hover {
    color: #fff;
  }
  
  .modal-container {
    background-color: #2a2a2a;
    color: #f0f0f0;
  }
  
  .modal-header {
    background-color: #1a1a1a;
  }
}

@media (max-width: 600px) {
  .controls {
    flex-direction: column;
    align-items: stretch;
  }
  
  .search-container {
    max-width: 100%;
  }
  
  .applicants {
    grid-template-columns: 1fr;
  }
}
</style>