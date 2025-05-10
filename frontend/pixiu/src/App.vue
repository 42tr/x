<script setup lang="ts">
import { onMounted, ref } from 'vue'
import Property from './components/Property.vue'
import Table from './components/Table.vue'
import EchartPie from './components/EchartPie.vue'
import { NSpace, NLayout, NLayoutContent, NLayoutFooter, NDatePicker, NGradientText, NNumberAnimation } from 'naive-ui'
import { getFundList, getDebtList, getPropertyList } from './api/api'
import type { Fund } from './types/fund'
import type { Debt } from './types/debt'
import type { Property as PropertyInfo } from './types/property'

const page_count = ref<number>(10)
const page = ref<number>(1)
const range = ref<[number, number]>([new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).getTime(), Date.now()])
const data = ref<Fund[]>([])
const debtData = ref<Debt[]>([])
const propertyData = ref<PropertyInfo[]>([])
const totalIncome = ref<number>(0)
const totalExpense = ref<number>(0)
const spendPieData = ref<
  {
    name: string
    value: number
  }[]
>([])

onMounted(async () => {
  await refresh()
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
  let resp = await getFundList(range.value[0], range.value[1], page.value, 10)
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
</script>

<template>
  <n-space vertical size="large" style="height: 100vh">
    <n-layout style="height: 100vh">
      <n-layout-content bordered content-style="padding: 24px;" style="height: 74.8%">
        <n-layout has-sider>
          <n-layout-content content-style="padding: 24px;">
            <div style="display: flex; align-items: center; gap: 8px">
              <n-date-picker
                v-model:value="range"
                type="daterange"
                size="small"
                style="width: 300px"
                :on-change="changeDate" />
              <n-gradient-text type="error" style="margin-left: 20px; width: 100px">
                收入：
                <n-number-animation
                  show-separator
                  ref="numberAnimationInstRef"
                  :from="0.0"
                  :to="totalIncome"
                  :precision="2" />
              </n-gradient-text>
              <n-gradient-text type="success">
                支出：
                <n-number-animation
                  show-separator
                  ref="numberAnimationInstRef"
                  :from="0.0"
                  :to="totalExpense"
                  :precision="2" />
              </n-gradient-text>
            </div>
            <br />
            <Table :data="data" />
            <n-pagination
              v-model:page="page"
              v-model:page-count="page_count"
              size="small"
              :on-change="refresh"
              v-if="page_count > 1" />
          </n-layout-content>
          <n-layout-sider
            content-style="padding: 24px; display: flex; justify-content: center; align-items: center"
            width="28.2%">
            <EchartPie :data="spendPieData" height="300px" width="300px" />
          </n-layout-sider>
        </n-layout>
      </n-layout-content>
      <n-layout-footer bordered style="padding: 20px">
        <Property :debtData="debtData" :propertyData="propertyData" style="height: 100%" />
      </n-layout-footer>
    </n-layout>
  </n-space>
</template>

<style scoped></style>
