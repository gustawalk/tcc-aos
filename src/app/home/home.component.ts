import { Component } from '@angular/core';
import { AuthService } from '../services/auth.service';
import { Router } from '@angular/router';
import { invoke } from '@tauri-apps/api/core';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent {

  constructor(private authService: AuthService, private router: Router) { }

  async tryCustomQuery() {
    const result = await invoke<String>('custom_query');
    console.log(result)
  }

  async logoutUser() {
    const sucess = await this.authService.logout();

    if (sucess) {
      alert("deslogado!");
      this.router.navigate(['/login']);
    }
    else {
      alert("erro em algo");
    }
  }

  forceLogin() {
    this.router.navigate(['/login']);
  }
}
