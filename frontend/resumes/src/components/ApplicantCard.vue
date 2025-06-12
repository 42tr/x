<script setup lang="ts">
import { computed, inject } from 'vue';
import { calculateGrade, calculateScore } from '../types/applicant';
import type { Applicant } from '../types/applicant';

interface CardProps {
  applicant: Applicant;
}

const props = defineProps<CardProps>();

// 获取父组件提供的弹窗控制方法
const openApplicantDetails = inject('openApplicantDetails') as (applicant: Applicant) => void;

// 计算平均分数
const averageScore = computed(() => {
  return calculateScore(props.applicant);
});

// 根据平均分数确定最终等级
const finalGrade = computed(() => {
  return calculateGrade(averageScore.value);
});

// 根据分数获取颜色
const getScoreColor = (score: number) => {
  if (score >= 8) return 'excellent';
  if (score >= 6) return 'good';
  if (score >= 4) return 'average';
  return 'poor';
};

// 获取等级颜色
const gradeColor = computed(() => {
  const average = averageScore.value;
  return getScoreColor(average);
});

// 点击卡片时打开详情
const showDetails = () => {
  openApplicantDetails(props.applicant);
};
</script>

<template>
  <div class="applicant-card-container">
    <div 
      class="applicant-card"
      @click="showDetails"
    >
      <div class="applicant-summary">
        <div class="applicant-name">{{ applicant.name }}</div>
        <div class="applicant-grade-container">
          <div class="applicant-score" :class="gradeColor">{{ averageScore.toFixed(1) }}</div>
          <div class="applicant-grade" :class="gradeColor">{{ finalGrade }}</div>
        </div>
      </div>
      
      <div class="card-hint">
        <small>点击查看详情</small>
      </div>
    </div>
  </div>
</template>

<style scoped>
.applicant-card-container {
  position: relative;
}

.applicant-card {
  padding: 1rem;
  border-radius: 8px;
  background-color: #f8f9fa;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
  cursor: pointer;
}

.applicant-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.applicant-summary {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.applicant-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: #2c3e50;
}

.applicant-grade-container {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.applicant-score {
  font-weight: bold;
  font-size: 1.25rem;
  padding: 0.5rem;
  border-radius: 4px;
  min-width: 2rem;
  text-align: center;
}

.applicant-grade {
  font-weight: bold;
  font-size: 1.25rem;
  padding: 0.5rem;
  border-radius: 4px;
  min-width: 2rem;
  text-align: center;
}

.card-hint {
  text-align: center;
  margin-top: 0.5rem;
  font-size: 0.75rem;
  color: #6c757d;
  opacity: 0.7;
}

/* 分数颜色 */
.excellent {
  background-color: #4caf50;
  color: white;
}

.good {
  background-color: #2196f3;
  color: white;
}

.average {
  background-color: #ff9800;
  color: white;
}

.poor {
  background-color: #f44336;
  color: white;
}

@media (prefers-color-scheme: dark) {
  .applicant-card {
    background-color: #2a2a2a;
    color: #f0f0f0;
  }
  
  .applicant-name {
    color: #e9ecef;
  }
  
  .card-hint {
    color: #adb5bd;
  }
}
</style>