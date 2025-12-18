export type read_acct_data_result = {
    readonly id: number
    readonly user_name: string
    readonly platform: string
    readonly remark: string
    readonly nonce_offset: number
    readonly use_up_letter: boolean
    readonly use_low_letter: boolean
    readonly use_number: boolean
    readonly use_sp_char: boolean
    readonly pwd_len: number
    readonly updated_at: string
}
