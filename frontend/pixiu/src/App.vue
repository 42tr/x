
<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import Property from './components/Property.vue'
import Table from './components/Table.vue'
import EchartPie from './components/EchartPie.vue'
import {
  NSpace,
  NLayout,
  NLayoutContent,
  NLayoutFooter,
  NDatePicker,
  NGradientText,
  NNumberAnimation,
  NButton,
  NModal,
  NCard,
  NInput,
  NSelect,
  NInputNumber
} from 'naive-ui'
import {
  getFundList,
  getDebtList,
  getPropertyList,
  getFundSources,
  getFundTypes,
  addFund,
  updateFund,
  deleteFund
} from './api/api'
import type { Fund } from './types/fund'
import type { Debt } from './types/debt'
import type { Property as PropertyInfo } from './types/property'

const page_count = ref<number>(10)
const page = ref<number>(1)
const range = ref<[number, number]>([
  new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).getTime(),
  Date.now()
])
const data = ref<Fund[]>([])
const debtData = ref<Debt[]>([])
const propertyData = ref<PropertyInfo[]>([])
const totalIncome = ref<number>(0)
const totalExpense = ref<number>(0)
const source = ref<string>('')
const type = ref<string[]>([])
const sources = ref<string[]>([])
const types = ref<string[]>([])
const spendPieData = ref<
  {
    name: string
    value: number
  }[]
>([])
const showModal = ref(false)
const currentFund = ref<Fund>({} as Fund)
const isEdit = ref(false)

watch([source, type], () => {
  changeDate()
})

onMounted(async () => {
  await refresh()
  sources.value = await getFundSources()
  types.value = await getFundTypes()
})

async function changeDate() {
  page.value = 1
  await refresh()
}

async function refresh() {
  await getFundInfo()
  await getDebtInfo()
  await getPropertyInfo()
}

async function getPropertyInfo() {
  let propertyList = await getPropertyList()
  propertyData.value = propertyList
}

async function getDebtInfo() {
  let resp = await getDebtList()
  let debtList = resp.map((item) => {
    if (item.last_timestamp) {
      item.last_date = new Date(item.last_timestamp).toISOString().split('T')[0]
    }
    return item
  })

  debtData.value = debtList
}

async function getFundInfo() {
  let resp = await getFundList(
    range.value[0],
    range.value[1],
    page.value,
    10,
    source.value,
    type.value.join(',')
  )
  page_count.value = Math.ceil(resp.total / 10)
  let fundList = resp.data.map((item) => {
    if (item.timestamp) {
      item.date = new Date(item.timestamp).toISOString().split('T')[0]
    }
    return item
  })

  data.value = fundList
  totalIncome.value = resp.income
  totalExpense.value = resp.expenses
  spendPieData.value = resp.sum
}

function handleAdd() {
  currentFund.value = { timestamp: Date.now() } as Fund
  isEdit.value = false
  showModal.value = true
}

function handleEdit(row: Fund) {
  currentFund.value = { ...row }
  isEdit.value = true
  showModal.value = true
}

async function handleDelete(row: Fund) {
  await deleteFund(row.id)
  await refresh()
}

async function handleSave() {
  if (isEdit.value) {
    await updateFund(currentFund.value)
  } else {
    await addFund(currentFund.value)
  }
  showModal.value = false
  await refresh()
  sources.value = await getFundSources()
  types.value = await getFundTypes()
}
</script>

<template>
  <n-space vertical size="large" style="height: 100vh">
    <n-layout style="height: 100vh">
      <n-layout-content bordered content-style="padding: 24px;" style="height: 74.8%">
        <n-layout has-sider>
          <n-layout-content content-style="padding: 24px;">
            <div style="display: flex; align-items: center; gap: 8px">
              <n-button @click="handleAdd">Add</n-button>
              <n-date-picker
                v-model:value="range"
                type="daterange"
                size="small"
                style="width: 300px"
                :on-change="changeDate"
              />
              <n-gradient-text type="error" style="margin-left: 20px; width: 100px">
                收入：
                <n-number-animation
                  show-separator
                  ref="numberAnimationInstRef"
                  :from="0.0"
                  :to="totalIncome"
                  :precision="2"
                />
              </n-gradient-text>
              <n-gradient-text type="success">
                支出：
                <n-number-animation
                  show-separator
                  ref="numberAnimationInstRef"
                  :from="0.0"
                  :to="totalExpense"
                  :precision="2"
                />
              </n-gradient-text>
            </div>
            <br />
            <Table
              :data="data"
              v-model:source="source"
              v-model:type="type"
              :sources="sources"
              :types="types"
              @edit="handleEdit"
              @delete="handleDelete"
            />
            <n-pagination
              v-model:page="page"
              v-model:page-count="page_count"
              size="small"
              :on-change="refresh"
              v-if="page_count > 1"
            />
          </n-layout-content>
          <n-layout-sider
            content-style="padding: 24px; display: flex; justify-content: center; align-items: center"
            width="28.2%"
          >
            <EchartPie :data="spendPieData" height="300px" width="300px" />
          </n-layout-sider>
        </n-layout>
      </n-layout-content>
      <n-layout-footer bordered style="padding: 20px">
        <Property :debtData="debtData" :propertyData="propertyData" style="height: 100%" />
      </n-layout-footer>
    </n-layout>
  </n-space>
  <n-modal v-model:show="showModal">
    <n-card style="width: 600px" :title="isEdit ? 'Edit Fund' : 'Add Fund'" :bordered="false" size="huge">
      <n-space vertical>
        <n-input v-model:value="currentFund.name" placeholder="Name" />
        <n-input-number v-model:value="currentFund.amount" placeholder="Amount" />
        <n-date-picker v-model:value="currentFund.timestamp" type="date" />
        <n-select
          v-model:value="currentFund.class"
          :options="types.map((item) => ({ label: item, value: item }))"
          placeholder="Type"
          tag
          filterable
        />
        <n-select
          v-model:value="currentFund.source"
          :options="sources.map((item) => ({ label: item, value: item }))"
          placeholder="Source"
          tag
          filterable
        />
      </n-space>
      <template #footer>
        <n-button @click="handleSave">Save</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<style scoped></style>
