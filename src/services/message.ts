import { ElMessage } from 'element-plus'

export const showError = (err: string) => {
  ElMessage.error(err);
}
