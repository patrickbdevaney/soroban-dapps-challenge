import React, { Dispatch, SetStateAction } from 'react'
import styles from './style.module.css'

export interface InputProps {
  placeHolder: string
  setAmount: Dispatch<SetStateAction<number | undefined>>
  input: string
  setInput: Dispatch<SetStateAction<string>>
  campaignId?: string
}

export function AmountInput({ placeHolder, setAmount, input, setInput, campaignId }: InputProps) {
  const handleChange = (event: {
    target: { name: string; value: string }
  }): void => {
    setAmount(parseInt(event.target.value))
    setInput(event.target.value)
    campaignId && setCampaignId(campaignId)
  }

  return (
    <input
      name="amount"
      type="number"
      placeholder={placeHolder}
      className={styles.input}
      onChange={handleChange}
      value={input}
      min={0}
      autoComplete="off"
    />
  )
}