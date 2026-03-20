<template>
  <div class="device-scanner">
    <h3>扫描设备</h3>
    <div class="actions">
      <button 
        @click="scanDevices" 
        :disabled="isScanning"
        class="btn btn-primary"
      >
        {{ isScanning ? '扫描中...' : '开始扫描' }}
      </button>
    </div>
    
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
    
    <div class="device-list" v-if="devices.length > 0">
      <h4>可用设备</h4>
      <ul>
        <li v-for="device in devices" :key="device.address">
          <div class="device-info">
            <span class="device-name">{{ device.name }}</span>
            <span class="device-address">{{ device.address }}</span>
          </div>
          <button 
            @click="connectDevice(device.address)" 
            :disabled="isConnecting"
            class="btn btn-success"
          >
            连接
          </button>
        </li>
      </ul>
    </div>
    
    <div v-else-if="!isScanning" class="no-devices">
      暂无设备，点击开始扫描
    </div>
  </div>
</template>

<script setup lang="ts">
import { useBluetoothStore } from '../../presentation/stores';

const bluetoothStore = useBluetoothStore();

const { 
  devices, 
  isScanning, 
  isConnecting, 
  error,
  scanDevices, 
  connectDevice 
} = bluetoothStore;
</script>

<style scoped>
.device-scanner {
  padding: 20px;
  background: #f5f5f5;
  border-radius: 8px;
  margin-bottom: 20px;
}

.actions {
  margin-bottom: 20px;
}

.error-message {
  color: #dc3545;
  margin: 10px 0;
  padding: 10px;
  background: #f8d7da;
  border-radius: 4px;
}

.device-list {
  margin-top: 20px;
}

.device-list h4 {
  margin-bottom: 10px;
  color: #333;
}

.device-list ul {
  list-style: none;
  padding: 0;
}

.device-list li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  border-bottom: 1px solid #ddd;
}

.device-info {
  flex: 1;
}

.device-name {
  font-weight: bold;
  margin-right: 10px;
}

.device-address {
  font-size: 12px;
  color: #666;
}

.no-devices {
  text-align: center;
  color: #666;
  padding: 20px;
  background: #fff;
  border-radius: 4px;
}

.btn {
  padding: 8px 16px;
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

.btn-success {
  background: #28a745;
  color: white;
}

.btn-success:hover {
  background: #218838;
}

.btn-success:disabled {
  background: #6c757d;
  cursor: not-allowed;
}
</style>