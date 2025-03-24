import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

@Injectable({
  providedIn: 'root'
})

export class AuthService {
  private isAuthenticated = false;

  constructor() {
    this.checkAuth();
  }

  async checkAuth() {
    try {
      const result = await invoke<boolean>('is_logged_in');
      this.isAuthenticated = result;
    } catch (error) {
      console.error('Error checking auth: ', error)
      this.isAuthenticated = false;
    }
  }

  async register(cpf: string, email: string, password: string): Promise<boolean> {
    try {
      const sucess = await invoke<boolean>('register', { cpf, email, password });
      if (sucess) {
        console.log("registro deu certo");
        this.isAuthenticated = true;
      }
      return sucess;
    } catch (error) {
      console.error('Register error: ', error);
      return false;
    }
  }

  async login(email: string, password: string): Promise<boolean> {
    try {
      const success = await invoke<boolean>('login', { email, password });
      if (success) {
        console.log('deu certo o login')
        this.isAuthenticated = true;
      }
      return success;
    } catch (error) {
      console.error('Login error:', error);
      return false;
    }
  }

  async logout(): Promise<boolean> {
    try {
      await invoke<boolean>('logout');
      this.isAuthenticated = false;
    } catch (error) {
      console.error('Logout error:', error);
    }
    return true;
  }

  isLogged(): boolean {
    return this.isAuthenticated;
  }
}
