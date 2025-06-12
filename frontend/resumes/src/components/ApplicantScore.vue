<script setup lang="ts">
import { computed } from 'vue'
import { calculateGrade, calculateScore } from '../types/applicant'
import type { Applicant } from '../types/applicant'
import ScoreChart from './ScoreChart.vue'

interface ScoreProps {
  applicant: Applicant
}

const props = defineProps<ScoreProps>()

// 计算平均分数
const averageScore = computed(() => {
  return calculateScore(props.applicant)
})

// 根据平均分数确定最终等级
const finalGrade = computed(() => {
  return calculateGrade(averageScore.value)
})

// 根据分数获取颜色
const getScoreColor = (score: number) => {
  if (score >= 8) return 'excellent'
  if (score >= 6) return 'good'
  if (score >= 4) return 'average'
  return 'poor'
}

// 获取等级颜色
const gradeColor = computed(() => {
  const average = averageScore.value
  return getScoreColor(average)
})
</script>

<template>
  <div class="applicant-score">
    <div class="score-card">
      <div class="summary-row">
        <div class="average-score">
          <div class="average-label">平均分数：</div>
          <div class="average-value" :class="gradeColor">{{ averageScore.toFixed(1) }}</div>
        </div>

        <div class="final-grade">
          <div class="grade-label">最终等级：</div>
          <div class="grade-value" :class="gradeColor">{{ finalGrade }}</div>
        </div>
      </div>

      <ScoreChart :applicant="applicant" />
    </div>
  </div>
</template>

<style scoped>
.applicant-score {
  background-color: #f8f9fa;
  color: #333;
  width: 100%;
}

.score-card {
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.summary-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin-bottom: 0.5rem;
}

.score-items {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
  padding: 1rem;
  background-color: rgba(255, 255, 255, 0.5);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.score-item {
  display: flex;
  justify-content: space-between;
  padding: 0.75rem;
  background-color: #fff;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.label {
  font-weight: 600;
}

.score {
  font-weight: bold;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  min-width: 2rem;
  text-align: center;
}

.average-score,
.final-grade {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.average-label,
.grade-label {
  font-weight: 600;
  font-size: 1.1rem;
}

.average-value,
.grade-value {
  font-weight: bold;
  font-size: 1.5rem;
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  min-width: 3rem;
  text-align: center;
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
  .applicant-score {
    background-color: #2a2a2a;
    color: #f0f0f0;
  }

  .score-items {
    background-color: rgba(255, 255, 255, 0.05);
  }

  .score-item,
  .average-score,
  .final-grade {
    background-color: #333;
    color: #f0f0f0;
  }
}

@media (max-width: 600px) {
  .summary-row {
    grid-template-columns: 1fr;
  }
}
</style>
