import http from './http'
import type { Fund } from '../types/fund'
import type { Debt } from '../types/debt'
import type { Page } from '../types/page'
import type { Property } from '../types/property'

export function getFundList(
  from: number,
  to: number,
  page: number,
  size: number,
  source?: string[],
  type?: string[],
  name?: string
): Promise<Page<Fund>> {
  let url = `/pixiu/fund?from=${from}&to=${to}&page=${page}&size=${size}`
  if (source && source.length > 0) {
    url += `&source=${source.join(',')}`
  }
  if (type && type.length > 0) {
    url += `&type=${type.join(',')}`
  }
  if (name) {
    url += `&name=${name}`
  }
  return http.get(url)
}

export function getFundSources(): Promise<string[]> {
  return http.get('/pixiu/fund/sources')
}

export function getFundTypes(): Promise<string[]> {
  return http.get('/pixiu/fund/types')
}

export function getDebtList(): Promise<Debt[]> {
  return http.get('/pixiu/debt')
}

export function getPropertyList(): Promise<Property[]> {
  return http.get('/pixiu/property')
}

export function addFund(fund: Fund): Promise<Fund> {
  return http.post('/pixiu/fund', fund)
}

export function updateFund(fund: Fund): Promise<Fund> {
  return http.put(`/pixiu/fund/${fund.id}`, fund)
}

export function deleteFund(id: number): Promise<void> {
  return http.delete(`/pixiu/fund/${id}`)
}
