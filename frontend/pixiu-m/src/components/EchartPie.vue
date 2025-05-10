<script lang="ts" setup>
import BaseEcharts from './baseEcharts/index.vue'
import { computed } from 'vue'

const props = defineProps({
  data: {
    type: Array,
    required: true
  },
  height: {
    type: String,
    default: '300px'
  },
  width: {
    type: String,
    default: '800px'
  }
})

const chartOptions = computed(() => {
  return {
    tooltip: {
      trigger: 'item',
      formatter: function (params: any) {
        return `${params.name}: ${params.value} (${params.percent}%)`
      }
    },
    legend: {
      top: '5%',
      left: 'center',
      show: false
    },
    series: [
      {
        type: 'pie',
        radius: ['40%', '70%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 10,
          borderColor: '#fff',
          borderWidth: 2
        },
        label: {
          show: true,
          formatter: '{b}: {c}', // b 是 name，c 是 value
          position: 'inside' // 或 'inside'
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 40,
            fontWeight: 'bold'
          }
        },
        labelLine: {
          show: false
        },
        data: props.data
      }
    ]
  }
})
</script>

<template>
  <BaseEcharts :options="chartOptions" :height="props.height" :width="props.width" />
</template>
