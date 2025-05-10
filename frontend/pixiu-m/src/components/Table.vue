<template>
  <n-data-table :columns="tableColumns" :data="props.data" />
</template>

<script lang="ts" setup>
import type { DataTableColumns } from 'naive-ui'
import { NGradientText } from 'naive-ui'
import { h } from 'vue'
import type { PropType } from 'vue'

interface Song {
  amount: number
  name: string
  class: string
  date: string
  source: string
}
const props = defineProps({
  data: { type: Array as PropType<Song[]>, required: true }
})

type GradientType = 'info' | 'success' | 'warning' | 'error' | 'primary' | 'danger'
function colorTitle(title: string, type: GradientType = 'info') {
  return h(
    NGradientText,
    {
      size: 20,
      type: type
    },
    { default: () => title }
  )
}

const tableColumns: DataTableColumns<Song> = [
  {
    key: 'amount',
    title() {
      return colorTitle('金额', 'danger')
    }
  },
  {
    key: 'name',
    title() {
      return colorTitle('名称', 'success')
    }
  },
  {
    key: 'class',
    title() {
      return colorTitle('类型', 'info')
    }
  },
  {
    key: 'date',
    title() {
      return colorTitle('日期', 'warning')
    }
  },
  {
    key: 'source',
    title() {
      return colorTitle('来源', 'error')
    }
  }
]
</script>
