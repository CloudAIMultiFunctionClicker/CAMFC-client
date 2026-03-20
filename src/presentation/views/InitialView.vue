<template>
  <div class="initial-view">
    <h1>CAMFC Cloud</h1>
    <p>欢迎使用CAMFC Cloud应用</p>
    
    <div class="connection-status">
      <h3>连接状态</h3>
      <div class="status-card">
        <div class="status-indicator" :class="connectionStateClass"></div>
        <div class="status-text">{{ connectionStatus }}</div>
      </div>
    </div>
    
    <DeviceScanner />
    
    <div v-if="isConnected" class="connected-section">
      <TotpDisplay />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useBluetoothStore } from '../stores';
import DeviceScanner from '../../components/DeviceScanner.vue';
import TotpDisplay from '../../components/TotpDisplay.vue';

const bluetoothStore = useBluetoothStore();

const { connectionState, connectionStatus, isConnected } = bluetoothStore;

const connectionStateClass = computed(() => {
  switch (connectionState) {
    case 'connected':
      return 'status-connected';
    case 'connecting':
      return 'status-connecting';
    default:
      return 'status-disconnected';
  }
});
</script>

<style scoped>
.initial-view {
  max-width: 800px;
  margin: 0 auto;
  padding: 40px 20px;
  text-align: center;
}

h1 {
  color: #333;
  margin-bottom: 10px;
}

p {
  color: #666;
  margin-bottom: 30px;
}

.connection-status {
  margin-bottom: 30px;
}

.status-card {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 8px;
  margin-bottom: 20px;
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.status-connected {
  background: #28a745;
}

.status-connecting {
  background: #ffc107;
  animation: pulse 1.5s infinite;
}

.status-disconnected {
  background: #dc3545;
}

@keyframes pulse {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
  100% {
    opacity: 1;
  }
}

.status-text {
  font-size: 16px;
  font-weight: 500;
  color: #333;
}

.connected-section {
  margin-top: 40px;
}
</style>