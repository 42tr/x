<template>
  <n-data-table :columns="tableColumns" :data="props.data" />
</template>

<script lang="ts" setup>
import type { DataTableColumns } from 'naive-ui'
import { NGradientText, NSelect, NButton, NSpace } from 'naive-ui'
import { h } from 'vue'
import type { PropType } from 'vue'

interface Song {
  id: number
  amount: number
  name: string
  class: string
  date: string
  source: string
}
const props = defineProps({
  data: { type: Array as PropType<Song[]>, required: true },
  sources: { type: Array as PropType<string[]>, required: true },
  types: { type: Array as PropType<string[]>, required: true }
})

const emit = defineEmits(['edit', 'delete'])

const source = defineModel('source')
const type = defineModel('type')

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
      return h(
        NSelect,
        {
          options: props.types.map((item) => ({ label: item, value: item })),
          value: type.value as string,
          'onUpdate:value': (v) => (type.value = v),
          placeholder: '选择类型',
          clearable: true,
          multiple: true
        },
        { default: () => colorTitle('类型', 'info') }
      )
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
      return h(
        NSelect,
        {
          options: props.sources.map((item) => ({ label: item, value: item })),
          value: source.value as string,
          'onUpdate:value': (v) => (source.value = v),
          placeholder: '选择来源',
          clearable: true
        },
        { default: () => colorTitle('来源', 'error') }
      )
    }
  },
  {
    key: 'actions',
    title: 'Actions',
    render(row) {
      return h(NSpace, {}, () => [
        h(
          NButton,
          {
            strong: true,
            tertiary: true,
            size: 'small',
            onClick: () => emit('edit', row)
          },
          { default: () => 'Edit' }
        ),
        h(
          NButton,
          {
            strong: true,
            tertiary: true,
            size: 'small',
            onClick: () => emit('delete', row)
          },
          { default: () => 'Delete' }
        )
      ])
    }
  }
]
</script>
