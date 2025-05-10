<script lang="ts" setup>
import { NThing, NAvatar, NIcon, NCard } from 'naive-ui'
import { Money } from '@vicons/carbon'
import { computed } from 'vue'
import type { PropType } from 'vue'

interface Debt {
  name: string
  amount: number
  repayment: number
  last_date: string
}
interface Property {
  name: string
  amount: number
}

const props = defineProps({
  debtData: {
    type: Array as PropType<Debt[]>,
    required: true
  },
  propertyData: {
    type: Array as PropType<Property[]>,
    required: true
  }
})
const debtTotal = computed(() => props.debtData.reduce((acc, cur) => acc + cur.amount, 0))
const propertyTotal = computed(() => props.propertyData.reduce((acc, cur) => acc + cur.amount, 0))
</script>

<template>
  <n-thing>
    <template #avatar>
      <n-avatar :size="25">
        <n-icon>
          <Money />
        </n-icon>
      </n-avatar>
    </template>
    <template #header>
      家庭负债
      <n-number-animation show-separator ref="numberAnimationInstRef" :from="0.0" :to="debtTotal" :precision="2" />
    </template>
    <template #footer>
      <div class="card-container">
        <n-card v-for="data in props.debtData" class="debtCard">
          <template #header>
            {{ data.name }}
            <n-number-animation
              show-separator
              ref="numberAnimationInstRef"
              :from="0.0"
              :to="data.amount"
              :precision="2" />
          </template>
          <template #default>
            最后日期：{{ data.last_date }}
            <br />
            还款金额：{{ data.repayment }}
          </template>
        </n-card>
      </div>
    </template>
  </n-thing>
  <br />
  <n-thing>
    <template #avatar>
      <n-avatar :size="25">
        <n-icon>
          <Money />
        </n-icon>
      </n-avatar>
    </template>
    <template #header>
      家庭资产
      <n-number-animation show-separator ref="numberAnimationInstRef" :from="0.0" :to="propertyTotal" :precision="2" />
    </template>
    <template #footer>
      <div class="card-container">
        <n-card v-for="data in props.propertyData" class="propertyCard">
          <template #header>
            {{ data.name }}
            <n-number-animation
              show-separator
              ref="numberAnimationInstRef"
              :from="0.0"
              :to="data.amount"
              :precision="2" />
          </template>
        </n-card>
      </div>
    </template>
  </n-thing>
</template>

<style scoped>
.card-container {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
}
.debtCard {
  width: 100%;
  height: 140px;
  display: flex;
}
.propertyCard {
  width: 100%;
  height: 70px;
  display: flex;
}
</style>
