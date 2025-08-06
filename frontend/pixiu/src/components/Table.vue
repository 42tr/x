<template>
  <n-data-table :columns="tableColumns" :data="props.data" />
</template>

<script lang="ts" setup>
import type { DataTableColumns } from 'naive-ui'
import { NGradientText, NSelect, NButton, NSpace, useDialog, NInput, NTooltip } from 'naive-ui'
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
const dialog = useDialog()

const source = defineModel<string[]>('source')
const type = defineModel<string[]>('type')
const name = defineModel<string>('name')

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
    key: 'class',
    width: 150,
    title() {
      return h(
        NSelect,
        {
          options: props.types.map((item) => ({ label: item, value: item })),
          value: type.value as string[],
          'onUpdate:value': (v: string[]) => (type.value = v),
          placeholder: '选择类型',
          clearable: true,
          multiple: true
        },
        { default: () => colorTitle('类型', 'info') }
      )
    },
    render(row) {
      return h(
        NTooltip,
        {},
        {
          trigger: () => h('span', {}, row.class),
          default: () => row.class
        }
      )
    }
  },
  {
    key: 'name',
    title() {
      return h(
        NInput,
        {
          value: name.value,
          'onUpdate:value': (v: string) => (name.value = v),
          placeholder: '输入名称',
          clearable: true
        },
        { default: () => colorTitle('名称', 'success') }
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
    width: 150,
    title() {
      return h(
        NSelect,
        {
          options: props.sources.map((item) => ({ label: item, value: item })),
          value: source.value as string[],
          'onUpdate:value': (v: string[]) => (source.value = v),
          placeholder: '选择来源',
          clearable: true,
          multiple: true
        },
        { default: () => colorTitle('来源', 'error') }
      )
    },
    render(row) {
      return h(
        NTooltip,
        {},
        {
          trigger: () => h('span', {}, row.source),
          default: () => row.source
        }
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
            onClick: () => {
              dialog.warning({
                title: '删除',
                content: '确认删除？',
                positiveText: '确认',
                negativeText: '取消',
                onPositiveClick: () => {
                  emit('delete', row)
                }
              })
            }
          },
          { default: () => 'Delete' }
        )
      ])
    }
  }
]
</script>
