<script setup lang="ts">
import { ref, onMounted, watch, onBeforeUnmount } from 'vue';
import type { Applicant } from '../types/applicant';

interface ChartProps {
  applicant: Applicant;
}

const props = defineProps<ChartProps>();
const chartCanvas = ref<HTMLCanvasElement | null>(null);
const resizeObserver = ref<ResizeObserver | null>(null);

// 为图例创建评分字段映射类型
type ScoreField = keyof Applicant['scores'];

// 绘制雷达图
const drawChart = () => {
  if (!chartCanvas.value) return;
  
  const ctx = chartCanvas.value.getContext('2d');
  if (!ctx) return;
  
  // 清除之前的绘图
  ctx.clearRect(0, 0, chartCanvas.value.width, chartCanvas.value.height);
  
  const scores = props.applicant.scores;
  const labels = ['工作经验', '教育背景', '面试表现', '技术能力', '文化契合度'];
  const values = [scores.experience, scores.education, scores.interview, scores.technical, scores.cultural];
  
  // 处理高分辨率屏幕
  const dpr = window.devicePixelRatio || 1;
  const rect = chartCanvas.value.getBoundingClientRect();
  
  // 设置画布尺寸并考虑设备像素比
  chartCanvas.value.width = rect.width * dpr;
  chartCanvas.value.height = rect.height * dpr;
  chartCanvas.value.style.width = `${rect.width}px`;
  chartCanvas.value.style.height = `${rect.height}px`;
  
  // 缩放绘图上下文以匹配设备像素比
  ctx.scale(dpr, dpr);
  
  // Chart configuration
  const centerX = rect.width / 2;
  const centerY = rect.height / 2;
  const radius = Math.min(centerX, centerY) - 30;
  const angleStep = (Math.PI * 2) / labels.length;
  
  // 绘制背景（圆形网格）
  for (let level = 1; level <= 10; level++) {
    const currentRadius = (radius / 10) * level;
    
    ctx.beginPath();
    ctx.arc(centerX, centerY, currentRadius, 0, Math.PI * 2);
    ctx.strokeStyle = level % 2 === 0 ? 'rgba(200, 200, 200, 0.5)' : 'rgba(200, 200, 200, 0.2)';
    ctx.stroke();
    
    if (level === 10 || level === 5) {
      ctx.fillStyle = 'rgba(150, 150, 150, 0.7)';
      ctx.font = '10px Arial';
      ctx.fillText(level.toString(), centerX + 5, centerY - currentRadius + 15);
    }
  }
  
  // 绘制轴线
  for (let i = 0; i < labels.length; i++) {
    const angle = i * angleStep - Math.PI / 2; // 从顶部开始（减去90度）
    const axisX = centerX + radius * Math.cos(angle);
    const axisY = centerY + radius * Math.sin(angle);
    
    ctx.beginPath();
    ctx.moveTo(centerX, centerY);
    ctx.lineTo(axisX, axisY);
    ctx.strokeStyle = 'rgba(200, 200, 200, 0.5)';
    ctx.stroke();
    
    // 绘制标签
    const labelX = centerX + (radius + 20) * Math.cos(angle);
    const labelY = centerY + (radius + 20) * Math.sin(angle);
    
    ctx.font = 'bold 12px Arial';
    ctx.fillStyle = '#555';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(labels[i], labelX, labelY);
  }
  
  // 绘制数据点并连接它们
  ctx.beginPath();
  for (let i = 0; i < values.length; i++) {
    const value = values[i];
    const angle = i * angleStep - Math.PI / 2; // 从顶部开始
    const pointRadius = (radius / 10) * value;
    const pointX = centerX + pointRadius * Math.cos(angle);
    const pointY = centerY + pointRadius * Math.sin(angle);
    
    if (i === 0) {
      ctx.moveTo(pointX, pointY);
    } else {
      ctx.lineTo(pointX, pointY);
    }
  }
  
  // 闭合路径回到第一个点
  const firstAngle = -Math.PI / 2; // 从顶部开始
  const firstPointRadius = (radius / 10) * values[0];
  const firstPointX = centerX + firstPointRadius * Math.cos(firstAngle);
  const firstPointY = centerY + firstPointRadius * Math.sin(firstAngle);
  ctx.lineTo(firstPointX, firstPointY);
  
  // 填充半透明颜色
  ctx.fillStyle = 'rgba(66, 184, 131, 0.5)'; // Vue绿色带透明度
  ctx.fill();
  
  // 描边轮廓
  ctx.strokeStyle = 'rgba(66, 184, 131, 0.8)';
  ctx.lineWidth = 2;
  ctx.stroke();
  
  // 在每个顶点绘制点
  for (let i = 0; i < values.length; i++) {
    const value = values[i];
    const angle = i * angleStep - Math.PI / 2;
    const pointRadius = (radius / 10) * value;
    const pointX = centerX + pointRadius * Math.cos(angle);
    const pointY = centerY + pointRadius * Math.sin(angle);
    
    ctx.beginPath();
    ctx.arc(pointX, pointY, 5, 0, Math.PI * 2);
    ctx.fillStyle = '#42b883'; // Vue绿色
    ctx.fill();
    ctx.strokeStyle = '#35495e'; // Vue深蓝色
    ctx.lineWidth = 1.5;
    ctx.stroke();
    
    // 添加分数文本
    ctx.font = 'bold 12px Arial';
    ctx.fillStyle = '#333';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    const textX = centerX + (pointRadius + 15) * Math.cos(angle);
    const textY = centerY + (pointRadius + 15) * Math.sin(angle);
    ctx.fillText(value.toString(), textX, textY);
  }
};

// 监听应聘者数据变化
watch(() => props.applicant, drawChart, { deep: true });

// 处理图表的响应式更新
const setupResizeObserver = () => {
  if (chartCanvas.value && 'ResizeObserver' in window) {
    resizeObserver.value = new ResizeObserver(() => {
      requestAnimationFrame(drawChart);
    });
    resizeObserver.value.observe(chartCanvas.value);
  }
};

// 在组件挂载时初始化图表
onMounted(() => {
  if (chartCanvas.value) {
    // 设置ResizeObserver以监听容器大小变化
    setupResizeObserver();
    
    // 初始绘制
    setTimeout(drawChart, 100); // 短暂延迟确保容器已完全渲染
    
    // 备用方案：窗口大小变化时重绘
    window.addEventListener('resize', drawChart);
  }
});

// 在组件卸载前清理资源
onBeforeUnmount(() => {
  // 清理ResizeObserver
  if (resizeObserver.value) {
    resizeObserver.value.disconnect();
  }
  // 移除事件监听器
  window.removeEventListener('resize', drawChart);
});
</script>

<template>
  <div class="chart-container">
    <h3>专业能力分析</h3>
    <div class="canvas-wrapper">
      <canvas ref="chartCanvas"></canvas>
    </div>
    
    <div class="score-legend">
      <div class="legend-item" v-for="(key, index) in [
        { name: '工作经验', field: 'experience' as ScoreField },
        { name: '教育背景', field: 'education' as ScoreField },
        { name: '面试表现', field: 'interview' as ScoreField },
        { name: '技术能力', field: 'technical' as ScoreField },
        { name: '文化契合度', field: 'cultural' as ScoreField }
      ]" :key="index">
        <div class="legend-color"></div>
        <div class="legend-label">{{ key.name }}: <strong>{{ applicant.scores[key.field] }}</strong></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.chart-container {
  padding: 1rem;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  margin-top: 0.5rem;
}

h3 {
  text-align: center;
  margin-top: 0;
  margin-bottom: 0.75rem;
  color: #2c3e50;
  font-size: 1.1rem;
  font-weight: 600;
}

.canvas-wrapper {
  width: 100%;
  display: flex;
  justify-content: center;
  height: 280px;
}

canvas {
  max-width: 100%;
  height: 100%;
}

.score-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem 1rem;
  justify-content: center;
  margin-top: 1rem;
  padding-top: 0.75rem;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.legend-color {
  width: 10px;
  height: 10px;
  background-color: #42b883;
  border-radius: 50%;
}

.legend-label {
  font-size: 0.85rem;
  color: #555;
}

@media (prefers-color-scheme: dark) {
  .chart-container {
    background-color: #333;
  }
  
  h3 {
    color: #e9ecef;
  }
  
  .legend-label {
    color: #ddd;
  }
  
  .score-legend {
    border-top-color: rgba(255, 255, 255, 0.1);
  }
}

@media (max-width: 768px) {
  .canvas-wrapper {
    height: 250px;
  }
  
  .score-legend {
    flex-direction: column;
    align-items: flex-start;
    margin-left: 1rem;
  }
}

@media (min-width: 769px) and (max-width: 1024px) {
  .score-legend {
    justify-content: space-around;
  }
}
</style>