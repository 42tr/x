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
  source?: string,
  type?: string
): Promise<Page<Fund>> {
  let url = `/pixiu/fund?from=${from}&to=${to}&page=${page}&size=${size}`
  if (source) {
    url += `&source=${source}`
  }
  if (type) {
    url += `&type=${type}`
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
