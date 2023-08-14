<script lang="ts" context="module">
	import Swal from 'sweetalert2';

	export function showAlert(message: string) {
		Swal.fire({
			icon: 'success',
			title: 'Success!',
			text: message
		});
	}

	export function showErrorAlert(message: string) {
		Swal.fire({
			icon: 'error',
			title: 'Oops...',
			text: message
		});
	}
</script>
