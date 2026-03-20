<template>
  <div class="totp-display">
    <h3>TOTP验证码</h3>
    <div class="totp-container">
      <div class="totp-code" v-if="totp">
        {{ totp }}
      </div>
      <div class="loading" v-else-if="isLoading">
        获取中...
      </div>
      <div class="error" v-else-if="error">
        {{ error }}
      </div>
      <div class="not-connected" v-else>
        请先连接设备
      </div>
    </div>
    <div class="actions">
      <button 
        @click="refreshTotp" 
        :disabled="isLoading || !isConnected"
        class="btn btn-primary"
      >
        刷新
      </button>
    </div>
    <div class="device-info" v-if="currentDevice">
      <p>当前设备: {{ currentDevice.name }}</p>
      <p>设备地址: {{ currentDevice.address }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useBluetoothStore } from '../stores';

const bluetoothStore = useBluetoothStore();
const totp = ref<string>('');
const isLoading = ref<boolean>(false);
const error = ref<string>('');

const currentDevice = computed(() => bluetoothStore.currentDevice);
const isConnected = computed(() => bluetoothStore.isConnected);

const refreshTotp = async () => {
  try {
    isLoading.value = true;
    error.value = '';
    totp.value = await bluetoothStore.getTotp();
  } catch (err) {
    error.value = '获取TOTP失败';
  } finally {
    isLoading.value = false;
  }
};

onMounted(() => {
  if (bluetoothStore.isConnected) {
    refreshTotp();
  }
});
</script>

<style scoped>
.totp-display {
  padding: 20px;
  background: #f5f5f5;
  border-radius: 8px;
  margin-bottom: 20px;
}

.totp-container {
  margin: 20px 0;
  padding: 30px;
  background: #fff;
  border-radius: 8px;
  text-align: center;
  min-height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.totp-code {
  font-size: 36px;
  font-weight: bold;
  letter-spacing: 4px;
  color: #333;
  font-family: monospace;
}

.loading {
  color: #666;
  font-style: italic;
}

.error {
  color: #dc3545;
}

.not-connected {
  color: #6c757d;
  font-style: italic;
}

.actions {
  margin: 20px 0;
  text-align: center;
}

.device-info {
  margin-top: 20px;
  padding: 15px;
  background: #e9ecef;
  border-radius: 4px;
  font-size: 14px;
}

.device-info p {
  margin: 5px 0;
  color: #495057;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover {
  background: #0069d9;
}

.btn-primary:disabled {
  background: #6c757d;
  cursor: not-allowed;
}
</style>