import React from 'react';
import { StatusBar } from 'expo-status-bar';
import { Provider } from 'react-redux';
import { PersistGate } from 'redux-persist/integration/react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { View, StyleSheet } from 'react-native';
import { HimsPaperProvider, ActivityIndicator, Surface, Text } from '@open-hims/mobile';
import { createHimsStore } from '@open-hims/store/mobile';
import HomeScreen from './src/screens/HomeScreen';

const Stack = createNativeStackNavigator();

// Create store instance for mobile
const { store, persistor } = createHimsStore({
  platform: 'mobile',
  enableDevTools: __DEV__,
});

export default function App() {
  return (
    <HimsPaperProvider theme="auto">
      <Provider store={store}>
        <PersistGate 
          loading={
            <Surface style={styles.loadingContainer}>
              <ActivityIndicator size="large" />
              <Text style={styles.loadingText}>Loading HIMS...</Text>
            </Surface>
          } 
          persistor={persistor}
        >
          <NavigationContainer>
            <StatusBar style="auto" />
            <Stack.Navigator
              initialRouteName="Home"
              screenOptions={{
                headerStyle: {
                  backgroundColor: '#0066cc',
                },
                headerTintColor: '#fff',
                headerTitleStyle: {
                  fontWeight: 'bold',
                },
              }}
            >
              <Stack.Screen 
                name="Home" 
                component={HomeScreen} 
                options={{ title: 'Open HIMS Mobile' }} 
              />
            </Stack.Navigator>
          </NavigationContainer>
        </PersistGate>
      </Provider>
    </HimsPaperProvider>
  );
}

const styles = StyleSheet.create({
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f5f5f5',
  },
  loadingText: {
    marginTop: 16,
    fontSize: 16,
    color: '#666',
  },
});