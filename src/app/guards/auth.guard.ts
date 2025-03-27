import { Injectable, inject } from '@angular/core';
import { ActivatedRouteSnapshot, Router, RouterState, RouterStateSnapshot } from '@angular/router';
import { AuthService } from '../services/auth.service';

@Injectable({
  providedIn: 'root'
})

export class AuthGuard {
  private authService = inject(AuthService);
  private router = inject(Router);


  async canActivate(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Promise<boolean> {
    await this.authService.checkAuth();

    let isLogged = this.authService.isLogged();
    const routePath = state.url;

    if (!isLogged && routePath !== '/login' && routePath !== '/register') {
      this.router.navigate(['/login']);
      return false;
    }

    if (isLogged && (routePath === '/login' || routePath === '/register')) {
      this.router.navigate(['/home']);
      return false;
    }

    return true;
  }
}

