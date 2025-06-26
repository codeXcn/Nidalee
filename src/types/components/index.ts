// 组件相关类型定义
export interface BaseComponent {
  id?: string
  class?: string
}

export interface CardProps extends BaseComponent {
  title?: string
  description?: string
  loading?: boolean
}

export interface ButtonProps extends BaseComponent {
  variant?: 'default' | 'primary' | 'secondary' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
}
