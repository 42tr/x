<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import type { Applicant } from '../types/applicant';

interface ChartProps {
  applicant: Applicant;
}

const props = defineProps<ChartProps>();
const chartCanvas = ref<HTMLCanvasElement | null>(null);

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
  
  // Chart configuration
  const centerX = chartCanvas.value.width / 2;
  const centerY = chartCanvas.value.height / 2;
  const radius = Math.min(centerX, centerY) - 20;
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

// 在组件挂载时初始化图表
onMounted(() => {
  if (chartCanvas.value) {
    // 根据容器大小设置画布尺寸
    chartCanvas.value.width = chartCanvas.value.parentElement?.clientWidth || 300;
    chartCanvas.value.height = 300;
    
    // 绘制初始图表
    drawChart();
    
    // 处理窗口大小调整
    const handleResize = () => {
      if (chartCanvas.value) {
        chartCanvas.value.width = chartCanvas.value.parentElement?.clientWidth || 300;
        chartCanvas.value.height = 300;
        drawChart();
      }
    };
    
    window.addEventListener('resize', handleResize);
    
    // 在组件卸载时清理事件监听器
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }
});
</script>

<template>
  <div class="chart-container">
    <h3>能力雷达图</h3>
    <div class="canvas-wrapper">
      <canvas ref="chartCanvas"></canvas>
    </div>
  </div>
</template>

<style scoped>
.chart-container {
  padding: 1rem;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  margin-top: 1rem;
}

h3 {
  text-align: center;
  margin-top: 0;
  margin-bottom: 1rem;
  color: #2c3e50;
}

.canvas-wrapper {
  width: 100%;
  display: flex;
  justify-content: center;
}

canvas {
  max-width: 100%;
}

@media (prefers-color-scheme: dark) {
  .chart-container {
    background-color: #333;
  }
  
  h3 {
    color: #e9ecef;
  }
}
</style>