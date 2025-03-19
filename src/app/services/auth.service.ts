import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private isAuthenticated = false;

  login(email: string, senha: string): boolean {
    if (email === 'admin@admin.com' && senha === '1234') {
      this.isAuthenticated = true;
      return true;
    }
    return false;
  }

  logout() {
    this.isAuthenticated = false;
  }

  isLogged(): boolean {
    return this.isAuthenticated;
  }
}

