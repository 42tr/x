export interface Page<T> {
  total: number
  data: T[]
  sum: {
    name: string
    value: number
  }[]
  income: number
  expenses: number
}
