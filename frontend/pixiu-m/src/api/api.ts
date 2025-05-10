import http from './http'
import type { Fund } from '../types/fund'
import type { Debt } from '../types/debt'
import type { Page } from '../types/page'
import type { Property } from '../types/property'

export function getFundList(from: number, to: number, page: number, size: number): Promise<Page<Fund>> {
  return http.get(`/pixiu/fund?from=${from}&to=${to}&page=${page}&size=${size}`)
}

export function getDebtList(): Promise<Debt[]> {
  return http.get('/pixiu/debt')
}

export function getPropertyList(): Promise<Property[]> {
  return http.get('/pixiu/property')
}
