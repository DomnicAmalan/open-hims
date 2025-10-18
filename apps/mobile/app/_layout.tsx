import { Stack } from 'expo-router';
import { Provider } from 'react-redux';
import { HimsPaperProvider } from '@open-hims/mobile';
import { store } from '@open-hims/store/mobile';

export default function RootLayout() {
  return (
    <HimsPaperProvider theme="auto">
      <Provider store={store}>
        <Stack>
          <Stack.Screen 
            name="index" 
            options={{ 
              title: '🏥 Open HIMS',
              headerStyle: {
                backgroundColor: 'rgba(33, 150, 243, 0.9)',
              },
              headerTintColor: '#fff',
              headerTitleStyle: {
                fontWeight: 'bold',
              },
            }} 
          />
          <Stack.Screen 
            name="(tabs)" 
            options={{ 
              headerShown: false 
            }} 
          />
        </Stack>
      </Provider>
    </HimsPaperProvider>
  );
}