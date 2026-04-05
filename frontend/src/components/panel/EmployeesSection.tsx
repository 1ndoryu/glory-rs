import {AVAILABILITY_LABELS} from '../../api/assignment';
import {useEmpleados} from '../../hooks/useAsignaciones';
import './EmployeesSection.css';

export function EmployeesSection() {
    const {empleados, cargando} = useEmpleados();

    if (cargando) {
        return <div className="employeesSectionState">Cargando equipo...</div>;
    }

    if (empleados.length === 0) {
        return (
            <div className="employeesSectionState">
                Todavia no hay empleados configurados en el marketplace.
            </div>
        );
    }

    const disponibles = empleados.filter(employee => employee.availability === 'available').length;
    const saturados = empleados.filter(
        employee => employee.current_orders >= employee.max_concurrent_orders,
    ).length;
    const promedioRating = Math.round(
        (empleados.reduce((accumulator, employee) => accumulator + (employee.average_rating ?? 0), 0) /
            empleados.length) * 10,
    ) / 10;

    return (
        <section className="employeesSection">
            <div className="employeesSectionSummary">
                <MetricCard label="Miembros" value={String(empleados.length)} />
                <MetricCard label="Disponibles" value={String(disponibles)} />
                <MetricCard label="Saturados" value={String(saturados)} />
                <MetricCard label="Rating promedio" value={promedioRating > 0 ? promedioRating.toFixed(1) : 'Sin datos'} />
            </div>

            <div className="employeesSectionGrid">
                {empleados.map(employee => {
                    const carga = `${employee.current_orders}/${employee.max_concurrent_orders}`;
                    const especialidades = employee.specialties.filter(Boolean);

                    return (
                        <article key={employee.user_id} className="employeesCard">
                            <div className="employeesCardHeader">
                                <div>
                                    <h3 className="employeesCardTitle">{employee.email}</h3>
                                    <p className="employeesCardMeta">{AVAILABILITY_LABELS[employee.availability] || employee.availability}</p>
                                </div>
                                <span className={`employeesStatus employeesStatus--${employee.availability}`}>
                                    {AVAILABILITY_LABELS[employee.availability] || employee.availability}
                                </span>
                            </div>

                            <dl className="employeesStats">
                                <div>
                                    <dt>Carga activa</dt>
                                    <dd>{carga}</dd>
                                </div>
                                <div>
                                    <dt>Ordenes completadas</dt>
                                    <dd>{employee.total_completed_orders}</dd>
                                </div>
                                <div>
                                    <dt>Rating</dt>
                                    <dd>{employee.average_rating ? employee.average_rating.toFixed(1) : 'Sin reviews'}</dd>
                                </div>
                            </dl>

                            <div className="employeesSkillsBlock">
                                <p className="employeesSkillsTitle">Especialidades</p>
                                <div className="employeesSkillsList">
                                    {especialidades.length > 0 ? (
                                        especialidades.map(skill => (
                                            <span key={`${employee.user_id}-${skill}`} className="employeesSkillBadge">
                                                {skill}
                                            </span>
                                        ))
                                    ) : (
                                        <span className="employeesSkillBadge employeesSkillBadge--empty">
                                            Sin especialidades cargadas
                                        </span>
                                    )}
                                </div>
                            </div>
                        </article>
                    );
                })}
            </div>
        </section>
    );
}

function MetricCard({label, value}: {label: string; value: string}) {
    return (
        <article className="employeesMetricCard">
            <span className="employeesMetricLabel">{label}</span>
            <strong className="employeesMetricValue">{value}</strong>
        </article>
    );
}