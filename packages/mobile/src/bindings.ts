import { NativeModules, Platform } from 'react-native';

const LINKING_ERROR =
  `The package 'hims-core-sdk-react-native' doesn't seem to be linked. Make sure: \n\n` +
  Platform.select({ ios: "- Run 'cd ios && pod install'\n", default: '' }) +
  '- You rebuilt the app after installing the package\n' +
  '- You are not using Expo Go\n';

const HimsCoreSdk = NativeModules.HimsCoreSdk
  ? NativeModules.HimsCoreSdk
  : new Proxy(
      {},
      {
        get() {
          throw new Error(LINKING_ERROR);
        },
      }
    );

export interface HimsConfig {
  apiEndpoint: string;
  authToken?: string;
  enableLogging: boolean;
}

export interface HimsCore {
  initialize(): Promise<string>;
}

export class HimsCoreSDK {
  private core: HimsCore | null = null;

  constructor(private config: HimsConfig) {}

  async initialize(): Promise<string> {
    if (!this.core) {
      this.core = await HimsCoreSdk.createHimsCore(this.config);
    }
    return this.core!.initialize();
  }

  // FHIR Methods
  async createPatient(patientData: any): Promise<any> {
    return HimsCoreSdk.createPatient(patientData);
  }

  async getPatient(id: string): Promise<any> {
    return HimsCoreSdk.getPatient(id);
  }

  // HL7v2 Methods
  async parseHL7Message(message: string): Promise<any> {
    return HimsCoreSdk.parseHL7Message(message);
  }

  // DICOM Methods
  async parseDicomMetadata(file: string): Promise<any> {
    return HimsCoreSdk.parseDicomMetadata(file);
  }

  // ABDM Methods
  async initiateConsent(request: any): Promise<any> {
    return HimsCoreSdk.initiateConsent(request);
  }
}

export default HimsCoreSDK;