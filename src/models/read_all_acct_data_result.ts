export type read_all_acct_data_result = {
    readonly page_count: number
    readonly page_content: page_content_type[]
}

export type page_content_type = {
    readonly id: number
    readonly user_name: string
    readonly platform: string
}
